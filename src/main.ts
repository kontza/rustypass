import { createApp } from 'vue'

import '@/assets/main.css'
import { createPinia } from 'pinia'
import App from '@/App.vue'
// import useMockIPCIfEnabled from '@/mock/mocks'

// useMockIPCIfEnabled()
const app = createApp(App).use(createPinia()).mount('#app')
