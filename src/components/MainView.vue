<script setup lang="ts">
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api'
  import { useTraceStore } from '@/stores/trace'
  import { useFileStore } from '@/stores/file'
  import { ref, watch, computed, onMounted } from 'vue'
  import { onKeyStroke } from '@vueuse/core'

  async function startScanning(): Promise<void> {
    console.log('start_scanning')
    fileStore.clearStore()
    await invoke('start_scanning')
  }
  const MINIMUM_FILTER_LENGTH = 2
  const tracingStore = useTraceStore()
  const fileStore = useFileStore()
  const copiedToClipboard = ref(false)
  const secretFailed = ref(false)
  const LABEL_TIMEOUT = 5000
  // eslint-disable-next-line
  const traceUnlistener = await listen('TRACE', (evt: any) => {
    const msg = JSON.parse(evt.payload.message)
    tracingStore.appendTrace(msg.fields?.payload)
    console.info('TRACE', msg.fields?.payload)
  })
  // eslint-disable-next-line
  const itemUnlistener = await listen('ITEM_FOUND', (evt: any) => {
    fileStore.addFile(evt.payload.path)
  })
  // eslint-disable-next-line
  const secretFailedListener = await listen('SECRET_FAILED', (evt: any) => {
    secretFailed.value = true
    setTimeout(() => (secretFailed.value = false), 2 * LABEL_TIMEOUT)
  })
  // eslint-disable-next-line
  const secretReadyListener = await listen('SECRET_READY', (evt: any) => {
    copiedToClipboard.value = true
    setTimeout(() => (copiedToClipboard.value = false), LABEL_TIMEOUT)
  })
  const fileTable = ref()
  const filterInput = ref()
  const currentRow = ref(-1)
  const badRegExp = ref(false)
  const filteredFiles = computed(() => {
    if (filterText.value.length >= MINIMUM_FILTER_LENGTH) {
      try {
        const filter = new RegExp(filterText.value.replace(' ', '.*'), 'i')
        return fileStore.files.filter((file) => filter.test(file))
      } catch (error) {
        // Probably a bad regexp, return all files.
        return fileStore.files
      }
    } else {
      return fileStore.files
    }
  })
  const filterText = ref('')
  const clickListener = (index: number): void => {
    currentRow.value = index
    fileTable.value.focus()
  }

  onMounted(() => {
    filterInput.value.focus()
    void startScanning()
  })

  watch(
    () => filterText.value,
    (oldText: string, newText: string) => {
      if (oldText.length >= MINIMUM_FILTER_LENGTH) {
        try {
          // eslint-disable-next-line
          const trial = new RegExp(oldText)
          badRegExp.value = false
        } catch (error) {
          badRegExp.value = true
        }
      } else {
        badRegExp.value = false
      }
    }
  )

  onKeyStroke('ArrowDown', (e: KeyboardEvent) => {
    if (currentRow.value < filteredFiles.value.length - 1) {
      currentRow.value++
    } else {
      currentRow.value = filteredFiles.value.length - 1
    }
    fileTable.value.focus()
    filterInput.value.blur()
  })
  onKeyStroke('ArrowUp', (e: KeyboardEvent) => {
    if (currentRow.value > 0) {
      currentRow.value--
    } else {
      currentRow.value = 0
    }
    fileTable.value.focus()
    filterInput.value.blur()
  })
  onKeyStroke('Escape', (e: KeyboardEvent) => {
    filterText.value = ''
    currentRow.value = -1
    secretFailed.value = false
    copiedToClipboard.value = false
    filterInput.value.focus()
    void startScanning()
  })
  onKeyStroke('Enter', (e: KeyboardEvent) => {
    if (filteredFiles.value.length === 1) {
      currentRow.value = 0
    }
    if (currentRow.value >= 0) {
      void invoke('process_secret', {
        secret: filteredFiles.value[currentRow.value]
      })
    }
  })
</script>
<style scoped></style>
<template>
  <table ref="fileTable" class="table table-zebra table-compact w-full">
    <thead class="sticky top-0">
      <tr>
        <input
          autofocus
          class="w-full input input-bordered"
          placeholder="Enter search term (regex)"
          v-model="filterText"
          ref="filterInput"
          :class="{
            'input-error': badRegExp
          }"
        />
        <label class="label">
          <span v-if="badRegExp" class="label-text-alt"
            >Invalid regular expression
          </span>
          <span v-if="copiedToClipboard" class="label-text-alt"
            >{{ filteredFiles[currentRow] }}: copied to the clipboard</span
          >
          <span v-if="secretFailed" class="label-text-alt text-error"
            >{{ filteredFiles[currentRow] }}: failed to get the secret</span
          >
        </label>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(file, index) in filteredFiles"
        :key="index"
        :class="{ active: index === currentRow }"
        @click="clickListener(index)"
      >
        <td>{{ file }}</td>
      </tr>
    </tbody>
  </table>
</template>
