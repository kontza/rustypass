<script>
  import { onMount } from 'svelte'
  import { initialize, decrypt, startScanning } from './lib/service'
  import { writable, derived } from 'svelte/store'

  const MINIMUM_FILTER_LENGTH = 2
  const LABEL_TIMEOUT = 5000
  const DEFAULT_INPUT_CLASSES = 'w-full flex-none input input-bordered'
  let flags = writable({
    badRegExp: false,
    copiedToClipboard: false,
    secretFailed: false
  })
  let currentSelection = ''
  let errorInputClasses = DEFAULT_INPUT_CLASSES + ' input-error'
  let fileTable
  let filterInput
  let filterText = writable('')
  let foundFiles = writable([])
  let optionHeight
  let windowInnerHeight
  let selectSize = 10
  let filteredFiles = derived([foundFiles, filterText], ([$foundFiles, $filterText]) => {
    if ($filterText.length >= MINIMUM_FILTER_LENGTH) {
      try {
        const filter = new RegExp($filterText.replace(' ', '.*'), 'i')
        const rv = $foundFiles.filter((file) => filter.test(file))
        console.log('filtered', rv)
        return rv
      } catch (error) {
        // Probably a bad regexp, return all files.
        return $foundFiles
      }
    } else {
      return $foundFiles
    }
  })

  onMount(async () => {
    // Remove previous listeners. It seems that in tauri-dev mode onMount is called many times.
    document.removeEventListener('ITEM_FOUND', handleItemFound, { capture: true })
    document.removeEventListener('SECRET_FAILED', handleSecretFailed, { capture: true })
    document.removeEventListener('SECRET_READY', handleSecretReady, { capture: true })
    document.addEventListener('ITEM_FOUND', handleItemFound, { capture: true })
    document.addEventListener('SECRET_FAILED', handleSecretFailed, { capture: true })
    document.addEventListener('SECRET_READY', handleSecretReady, { capture: true })
    filterInput.focus()
    let listeners = initialize()
    filteredFiles.subscribe(recalculateSize)
    return () => {
      listeners.then((unlisteners) => {
        unlisteners.forEach((unlistener) => unlistener())
      })
    }
  })

  function recalculateSize() {
    let optionHeightToUse = optionHeight || 18
    let inputHeight = (filterInput || {}).clientHeight || 48
    let heightForSelect = windowInnerHeight - inputHeight
    selectSize = Math.floor(heightForSelect / optionHeightToUse)
    console.log('selectSize', selectSize, optionHeight, (filterInput || {}).clientHeight)
  }

  function clickListener() {
    decrypt(getSecretToOpen())
  }

  function handleKeyUp(event) {
    switch (event.key) {
      case 'Escape':
        $filterText = ''
        filterInput.focus()
        break

      case 'Enter':
        decrypt(getSecretToOpen())
        break

      default:
        // console.info('event:', event)
        break
    }
  }

  function handleItemFound(payload) {
    console.log('RCV item found', payload.detail)
    $foundFiles = [...$foundFiles, payload.detail]
  }

  function handleSecretFailed() {
    $flags.secretFailed = true
    setTimeout(() => ($flags.secretFailed = false), LABEL_TIMEOUT)
  }

  function handleSecretReady() {
    $flags.copiedToClipboard = true
    setTimeout(() => ($flags.copiedToClipboard = false), LABEL_TIMEOUT)
  }

  function getSecretToOpen() {
    if ($filteredFiles.length === 1) {
      return filteredFiles[0]
    } else {
      return currentSelection.trim()
    }
  }
</script>

<svelte:window
  on:resize={recalculateSize}
  on:keyup={handleKeyUp}
  bind:innerHeight={windowInnerHeight}
/>

<main>
  <div class="toast toast-top toast-end">
    {#if $flags.copiedToClipboard}
      <div class="alert alert-success">
        <span>Secret copied to the clipboard</span>
      </div>
    {/if}
    {#if $flags.secretFailed}
      <div class="alert alert-error">
        <span>Failed to get the secret</span>
      </div>
    {/if}
    {#if $flags.badRegExp}
      <div class="alert alert-error">
        <span>Invalid regular expression</span>
      </div>
    {/if}
  </div>

  <div class="h-full" style="flex-direction: column; display: flex">
    <input
      placeholder="Enter search term (regex)"
      bind:value={$filterText}
      bind:this={filterInput}
      class={$flags.badRegExp ? errorInputClasses : DEFAULT_INPUT_CLASSES}
    />
    <select
      bind:this={fileTable}
      bind:value={currentSelection}
      class="h-full select select-bordered"
      on:dblclick={clickListener}
      size={selectSize}
    >
      {#each $filteredFiles as file}
        <option bind:clientHeight={optionHeight}>
          {file}
        </option>
      {/each}
    </select>
  </div>
</main>

<style>
</style>
