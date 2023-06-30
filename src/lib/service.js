// @ts-check
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { isRegistered, register } from '@tauri-apps/api/globalShortcut'
import { envMocksEnabled, envTraceEnabled } from './env'
import { fillStore, useMockIPCIfEnabled } from './mocks'
import { tick } from 'svelte'

export async function decrypt(secret) {
  void invoke('process_secret', { secret: secret })
}

export async function initialize() {
  useMockIPCIfEnabled()
  let listeners = setUpListeners()
  void startScanning()
  return listeners
}

async function startScanning() {
  if (envMocksEnabled()) {
    fillStore().forEach((entry) =>
      document.dispatchEvent(new CustomEvent('ITEM_FOUND', { detail: entry }))
    )
  }
  await tick()
  await invoke('start_scanning')
  await invoke('get_global_shortcut')
}

async function setUpListeners() {
  return [
    await listen('ITEM_FOUND', (evt) =>
      document.dispatchEvent(new CustomEvent('ITEM_FOUND', { detail: evt.payload.path }))
    ),
    await listen('SECRET_FAILED', () => document.dispatchEvent(new Event('SECRET_FAILED'))),
    await listen('SECRET_READY', () => document.dispatchEvent(new Event('SECRET_READY'))),
    await listen('SHORTCUT', (evt) => setGlobalShortcut(evt.payload)),
    await listen('TRACE', (evt) => {
      const msg = JSON.parse(evt.payload.message)
      if (envTraceEnabled()) {
        console.info('TRACE', msg.fields?.payload)
      }
    })
  ]
}

async function setGlobalShortcut(shortcut) {
  let alreadyRegistered = await isRegistered(shortcut)
  if (!alreadyRegistered) {
    console.log('setting shortcut', shortcut)
    await register(shortcut, async () => {
      await appWindow.setFocus()
    }).catch((err) => {
      console.error('Shortcut registering failed', err)
    })
  }
}
