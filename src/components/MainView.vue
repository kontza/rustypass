<script setup lang="ts">
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api'
  import { useTraceStore } from '@/stores/trace'
  import { useFileStore } from '@/stores/file'
  import { ref, watch, computed, onMounted } from 'vue'

  async function startScanning(): Promise<void> {
    console.log('start_scanning')
    fileStore.clearStore()
    invoke('start_scanning')
  }
  const MINIMUM_FILTER_LENGTH = 2
  const tracingStore = useTraceStore()
  const fileStore = useFileStore()
  const unlisten = await listen('TRACE', (evt: any) => {
    const msg = JSON.parse(evt.payload.message)
    tracingStore.appendTrace(msg.fields?.payload)
  })
  const fileTable = ref()
  const currentRow = ref(-1)
  const badRegExp = ref(false)
  const filteredFiles = computed(() => {
    if (filterText.value.length >= MINIMUM_FILTER_LENGTH) {
      try {
        const filter = new RegExp(filterText.value, 'i')
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

  onMounted(() => {
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

  const escListener = (): void => {
    filterText.value = ''
  }

  const downListener = (): void => {
    console.error(
      '>>> currentRow',
      currentRow,
      'filteredFiles',
      filteredFiles.value.length
    )
    if (currentRow.value < filteredFiles.value.length) {
      currentRow.value++
    }
  }
</script>

<template>
  <div class="card" @keyup.down.prevent="downListener">
    <input
      class="w-full input input-bordered"
      placeholder="Enter search term (regex)"
      v-model="filterText"
      @keyup.esc="escListener"
      :class="{ 'input-error': badRegExp }"
    />
    <label class="label" v-if="badRegExp">
      <span class="label-text-alt">Invalid regular expression</span>
    </label>
    <table ref="fileTable" class="table table-zebra table-compact w-full">
      <tbody>
        <tr
          v-for="(file, index) in filteredFiles"
          :key="index"
          :class="{ active: index === currentRow }"
        >
          <td>{{ file }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
