import { mockIPC } from '@tauri-apps/api/mocks'
import { useFileStore } from '@/stores/file'

function fillStore(): void {
  const fileStore = useFileStore()

  for (const entry of [
    'gothse.gpg',
    'gibsonfirebreatha.gpg',
    'greenpayne.gpg',
    'berryhowla/barneclaw.gpg',
    'pointydaniels/grangriffiths.gpg',
    'jenkinininki.gpg',
    'dobanderson.gpg',
    'drogcole.gpg',
    'hahugia.gpg',
    'zimbradley.gpg',
    'pixlane/bateswood.gpg',
    'countess/shary/woofang.gpg',
    'gra-in-the-green.gpg',
    'harrificent.gpg',
    'goliread.gpg',
    'keblex.gpg',
    'phillipsskelisia.gpg',
    'munozow/pricedall.gpg',
    'evansdelia.gpg',
    'scro/reilly.gpg',
    'cheekfletcher.gpg',
    'gonzastinkie.gpg',
    'carroconda.gpg',
    'coth.gpg',
    'brofang/kendgreen.gpg',
    'clarktopi.gpg',
    'freena/brott.gpg',
    'grrter.gpg',
    'mccarthyyanka.gpg',
    'ramirezclop.gpg',
    'abadwards/satanpayne.gpg',
    'mendoslight.gpg',
    'gumpa.gpg',
    'wane/granganderson.gpg',
    'malyes.gpg',
    'chenievous.gpg',
    'reestenna/collinmoondancer.gpg',
    'marshantress/widore.gpg',
    'chavezgoodness.gpg',
    'edwabroom/fralline.gpg',
    'hayesilda/grangrdson.gpg',
    'jenkinsshine.gpg'
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
        case 'process_secret':
          console.info('>>> process_secret', args)
          break
        default:
          console.warn('>>> unknown cmd', cmd)
          break
      }
    })
  }
}
export default useMockIPCIfEnabled
