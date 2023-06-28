import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { envMocksEnabled, envTraceEnabled } from './env'
import { fillStore, useMockIPCIfEnabled } from './mocks'

export async function initialize() {
  console.log('initialize')
  useMockIPCIfEnabled()
  let listeners = setUpListeners()
  void startScanning()
  return listeners
}

export async function startScanning() {
  if (envMocksEnabled()) {
    fillStore().forEach((entry) =>
      document.dispatchEvent(new CustomEvent('ITEM_FOUND', { detail: entry }))
    )
  }
  console.log('start_scanning')
  setTimeout(async () => {
    await invoke('start_scanning')
  }, 0)
}

export async function setUpListeners() {
  return [
    await listen('ITEM_FOUND', (evt) => {
      console.log('SND item found', evt.payload.path)
      document.dispatchEvent(new CustomEvent('ITEM_FOUND', { detail: evt.payload.path }))
    }),
    await listen('SECRET_FAILED', () => {
      document.dispatchEvent(new Event('SECRET_FAILED'))
    }),
    await listen('SECRET_READY', () => {
      document.dispatchEvent(new Event('SECRET_READY'))
    }),
    await listen('TRACE', (evt) => {
      const msg = JSON.parse(evt.payload.message)
      if (envTraceEnabled()) {
        console.info('TRACE', msg.fields?.payload)
      }
    })
  ]
}

export async function decrypt(secret) {
  void invoke('process_secret', { secret: secret })
}
