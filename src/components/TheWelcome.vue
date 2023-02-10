<script setup lang="ts">
  import WelcomeItem from '@/components/WelcomeItem.vue'
  import DocumentationIcon from '@/components/icons/IconDocumentation.vue'
  import ToolingIcon from '@/components/icons/IconTooling.vue'
  import EcosystemIcon from '@/components/icons/IconEcosystem.vue'
  import CommunityIcon from '@/components/icons/IconCommunity.vue'
  import SupportIcon from '@/components/icons/IconSupport.vue'
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from '@tauri-apps/api'
  import { useTraceStore } from '@/stores/trace'
  import { useFileStore } from '@/stores/file'
  import { ref, computed } from 'vue'

  async function startScanning(): Promise<void> {
    console.log('start_scanning')
    fileStore.clearStore()
    invoke('start_scanning')
  }
  const tracingStore = useTraceStore()
  const fileStore = useFileStore()
  const unlisten = await listen('TRACE', (evt: any) => {
    const msg = JSON.parse(evt.payload.message)
    tracingStore.appendTrace(msg.fields?.payload)
  })
  const files = ref('Click Start scanning -button')
  fileStore.$subscribe((mutation, state) => {
    files.value = state.files.join('\n')
  })
  const allFiles = computed(() => files.value)
</script>

<template>
  <WelcomeItem>
    <button class="btn" type="button" @click="startScanning">
      Start scanning
    </button>
  </WelcomeItem>
  <WelcomeItem>
    <textarea
      class="textarea-success"
      id="traces"
      name="traces"
      rows="10"
      cols="80"
      readonly
      >{{ allFiles }}
      </textarea
    >
  </WelcomeItem>
</template>
