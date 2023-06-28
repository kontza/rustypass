<script>
  import { onMount } from 'svelte'
<<<<<<< HEAD
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api'
  import { envMocksEnabled, envTraceEnabled } from '@/lib/env'
  import { fillStore, useMockIPCIfEnabled } from '@/lib/mocks'
=======
  import { initialize, decrypt, startScanning } from './lib/service'
  import { writable, derived } from 'svelte/store'
>>>>>>> terzo

  const MINIMUM_FILTER_LENGTH = 2
  const LABEL_TIMEOUT = 5000
  const DEFAULT_INPUT_CLASSES = 'w-full flex-none input input-bordered'
<<<<<<< HEAD
  let badRegExp = false
  let copiedToClipboard = false
=======
  let flags = writable({
    badRegExp: false,
    copiedToClipboard: false,
    secretFailed: false
  })
>>>>>>> terzo
  let currentSelection = ''
  let errorInputClasses = DEFAULT_INPUT_CLASSES + ' input-error'
  let fileTable
  let filterInput
<<<<<<< HEAD
  let filterText = ''
  let foundFiles = []
  let filteredFiles = []
  let secretFailed = false
  let singleOption
  let windowInnerHeight
  let selectSize = 10

  $: filterFiles(filterText)
  $: calculateListSize(filteredFiles)

  function calculateListSize(list) {
    let optionHeight = 18
    let inputHeight = 48
    if (singleOption) {
      optionHeight = singleOption.clientHeight
      inputHeight = filterInput.clientHeight
    }
    let heightForSelect = windowInnerHeight - inputHeight
    selectSize = Math.floor(heightForSelect / optionHeight)
  }

  function filterFiles(listFilter) {
    if (listFilter.length >= MINIMUM_FILTER_LENGTH) {
      try {
        const filter = new RegExp(listFilter.replace(' ', '.*'), 'i')
        const rv = foundFiles.filter((file) => filter.test(file))
        if (rv.length === 1) {
          // rv.push('')
        }
        filteredFiles = rv
      } catch (error) {
        // Probably a bad regexp, return all files.
        filteredFiles = foundFiles
      }
    } else {
      filteredFiles = foundFiles
    }
  }

  async function startScanning() {
    if (envMocksEnabled()) {
      foundFiles = fillStore()
      filteredFiles = foundFiles
    } else {
      foundFiles = []
    }
    await invoke('start_scanning')
  }

  async function setUpListeners() {
    return [
      await listen('TRACE', (evt) => {
        const msg = JSON.parse(evt.payload.message)
        if (envTraceEnabled()) {
          console.info('TRACE', msg.fields?.payload)
        }
      }),
      await listen('ITEM_FOUND', (evt) => {
        foundFiles.push(evt.payload.path)
      }),
      await listen('SECRET_FAILED', () => {
        secretFailed = true
        setTimeout(() => (secretFailed = false), 2 * LABEL_TIMEOUT)
      }),
      await listen('SECRET_READY', () => {
        copiedToClipboard = true
        setTimeout(() => (copiedToClipboard = false), LABEL_TIMEOUT)
      })
    ]
  }

  onMount(async () => {
    filterInput.focus()
    useMockIPCIfEnabled()
    let listeners = setUpListeners()
    startScanning()
=======
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
>>>>>>> terzo
    return () => {
      listeners.then((unlisteners) => {
        unlisteners.forEach((unlistener) => unlistener())
      })
    }
  })

<<<<<<< HEAD
  function clickListener(index) {
    void invoke('process_secret', { secret: currentSelection.trim() })
=======
  function recalculateSize() {
    let optionHeightToUse = optionHeight || 18
    let inputHeight = (filterInput || {}).clientHeight || 48
    let heightForSelect = windowInnerHeight - inputHeight
    selectSize = Math.floor(heightForSelect / optionHeightToUse)
    console.log('selectSize', selectSize, optionHeight, (filterInput || {}).clientHeight)
  }

  function clickListener() {
    decrypt(getSecretToOpen())
>>>>>>> terzo
  }

  function handleKeyUp(event) {
    switch (event.key) {
      case 'Escape':
<<<<<<< HEAD
        filterText = ''
=======
        $filterText = ''
>>>>>>> terzo
        filterInput.focus()
        break

      case 'Enter':
<<<<<<< HEAD
        if (filteredFiles.length === 1) {
          void invoke('process_secret', { secret: filteredFiles[0] })
        } else {
          void invoke('process_secret', { secret: currentSelection.trim() })
        }
=======
        decrypt(getSecretToOpen())
>>>>>>> terzo
        break

      default:
        // console.info('event:', event)
        break
    }
  }
<<<<<<< HEAD
</script>

<svelte:window
  on:resize={calculateListSize}
=======

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
>>>>>>> terzo
  on:keyup={handleKeyUp}
  bind:innerHeight={windowInnerHeight}
/>

<main>
<<<<<<< HEAD
  <input
    placeholder="Enter search term (regex)"
    bind:value={filterText}
    bind:this={filterInput}
    class={badRegExp ? errorInputClasses : DEFAULT_INPUT_CLASSES}
  />
  <div class="h-full" style="flex-direction: column; display: flex">
=======
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
>>>>>>> terzo
    <select
      bind:this={fileTable}
      bind:value={currentSelection}
      class="h-full select select-bordered"
      on:dblclick={clickListener}
      size={selectSize}
    >
<<<<<<< HEAD
      {#each filteredFiles as file}
        <option bind:this={singleOption} disabled={file === ''}>
=======
      {#each $filteredFiles as file}
        <option bind:clientHeight={optionHeight}>
>>>>>>> terzo
          {file}
        </option>
      {/each}
    </select>
  </div>
</main>

<style>
</style>
