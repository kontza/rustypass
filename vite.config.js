import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  clearScreen: false,
  server: {
    strictPort: true,
    port: 9110
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG
  },
  plugins: [svelte()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})
