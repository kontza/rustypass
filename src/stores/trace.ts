import { defineStore } from 'pinia'

interface State {
  traces: string[]
}

export const useTraceStore = defineStore('trace', {
  state: (): State => ({
    traces: []
  }),
  getters: {
    getTraces: (state: State) => state.traces
  },
  actions: {
    appendTrace(trace: string) {
      this.traces?.push(trace)
    }
  }
})
