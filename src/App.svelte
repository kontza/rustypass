<script>
  // @ts-check
  import { onMount } from 'svelte'
  import { writable, derived } from 'svelte/store'
  import { initialize, decrypt } from './lib/service'

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
    const eventsAndListeners = new Map([
      ['ITEM_FOUND', handleItemFound],
      ['SECRET_FAILED', handleSecretFailed],
      ['SECRET_READY', handleSecretReady],
      ['SHORTCUT', setGlobalShortcut]
    ])
    for (let [eventName], eventListener] of eventsAndListeners) {
      // Remove previous listeners. It seems that in tauri-dev mode onMount is called many times.
      document.removeEventListener(eventName, eventListener)
    }
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

  function setGlobalShortcut(payload) {
    console.log('Would process shortcut', payload)
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
