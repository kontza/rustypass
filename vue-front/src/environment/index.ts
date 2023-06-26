export const envMocksEnabled = () =>
  import.meta.env.VITE_MOCK_TAURI_API?.toString().toLowerCase() === 'true'
export const envTraceEnabled = () =>
  import.meta.env.VITE_TRACE_ENABLED?.toString().toLowerCase() === 'true'
