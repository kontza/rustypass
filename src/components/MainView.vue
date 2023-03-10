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
  const filterInput = ref()
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

  const escListener = (): void => {
    filterText.value = ''
    currentRow.value = -1
    filterInput.value.focus()
  }

  const downListener = (): void => {
    console.error('>>> downListener')
    if (currentRow.value < filteredFiles.value.length - 1) {
      currentRow.value++
    } else {
      currentRow.value = filteredFiles.value.length - 1
    }
    fileTable.value.focus()
  }

  const upListener = (): void => {
    console.error('>>> upListener')
    if (currentRow.value > 0) {
      currentRow.value--
    } else {
      currentRow.value = 0
    }
    fileTable.value.focus()
  }

  const clickListener = (index: number): void => {
    console.error('>>> active', index)
    currentRow.value = index
    fileTable.value.focus()
  }
</script>

<template>
  <div
    class="card"
    @keydown.esc="escListener"
    @keydown.down="downListener"
    @keydown.up="upListener"
  >
    <input
      autofocus
      class="w-full input input-bordered"
      placeholder="Enter search term (regex)"
      v-model="filterText"
      ref="filterInput"
      :class="{ 'input-error': badRegExp }"
      @keydown.esc="escListener"
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
          @click="clickListener(index)"
          @keydown.down="downListener"
          @keyup="upListener"
        >
          <td>{{ file }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>
