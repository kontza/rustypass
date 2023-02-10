import { mockIPC } from '@tauri-apps/api/mocks'
import { useFileStore } from '@/stores/file'

function fillStore(): void {
  const fileStore = useFileStore()

  for (const entry of [
    'ansible/vault.gpg',
    'become.gpg',
    'ldap/2017.gpg',
    'viurvahti/become.gpg',
    'votsonkolo/become.gpg'
  ]) {
    fileStore.addFile(entry)
  }
}

export function useMockIPCIfEnabled(): void {
  if (import.meta.env.VITE_APP_MOCK_TAURI_API === 'true') {
    console.info('>>> MOCK_TAURI_API enabled')
    mockIPC((cmd, args) => {
      switch (cmd) {
        case 'tauri':
          switch (args.__tauriModule) {
            case 'Event':
              console.info('>>> Event', args.message)
              break
          }
          break
        case 'start_scanning':
          fillStore()
          break
        default:
          console.warn('>>> unknown cmd', cmd)
          break
      }
    })
  }
}
export default useMockIPCIfEnabled
