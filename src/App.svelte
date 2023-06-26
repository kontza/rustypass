<script>
  import { onMount } from 'svelte'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api'
  import { envMocksEnabled, envTraceEnabled } from '@/lib/env'
  import { fillStore, useMockIPCIfEnabled } from '@/lib/mocks'

  const MINIMUM_FILTER_LENGTH = 2
  const LABEL_TIMEOUT = 5000
  const DEFAULT_INPUT_CLASSES = 'w-full flex-none input input-bordered'
  let badRegExp = false
  let copiedToClipboard = false
  let currentSelection = ''
  let errorInputClasses = DEFAULT_INPUT_CLASSES + ' input-error'
  let fileTable
  let filterInput
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
    return () => {
      listeners.then((unlisteners) => {
        unlisteners.forEach((unlistener) => unlistener())
      })
    }
  })

  function clickListener(index) {
    void invoke('process_secret', { secret: currentSelection.trim() })
  }

  function handleKeyUp(event) {
    switch (event.key) {
      case 'Escape':
        filterText = ''
        filterInput.focus()
        break

      case 'Enter':
        if (filteredFiles.length === 1) {
          void invoke('process_secret', { secret: filteredFiles[0] })
        } else {
          void invoke('process_secret', { secret: currentSelection.trim() })
        }
        break

      default:
        // console.info('event:', event)
        break
    }
  }
</script>

<svelte:window
  on:resize={calculateListSize}
  on:keyup={handleKeyUp}
  bind:innerHeight={windowInnerHeight}
/>

<main>
  <input
    placeholder="Enter search term (regex)"
    bind:value={filterText}
    bind:this={filterInput}
    class={badRegExp ? errorInputClasses : DEFAULT_INPUT_CLASSES}
  />
  <div class="h-full" style="flex-direction: column; display: flex">
    <select
      bind:this={fileTable}
      bind:value={currentSelection}
      class="h-full select select-bordered"
      on:dblclick={clickListener}
      size={selectSize}
    >
      {#each filteredFiles as file}
        <option bind:this={singleOption} disabled={file === ''}>
          {file}
        </option>
      {/each}
    </select>
  </div>
</main>

<style>
</style>
