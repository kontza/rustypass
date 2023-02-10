import { mockIPC } from '@tauri-apps/api/mocks'

const useMockIPCIfEnabled = function (): void {
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
          console.info('>>> start_scanning', args)
          break
        default:
          console.warn('>>> unknown cmd', cmd)
          break
      }
    })
  }
}
export default useMockIPCIfEnabled
