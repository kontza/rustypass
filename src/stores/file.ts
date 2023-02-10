import { defineStore } from 'pinia'

interface State {
  files: string[]
}

export const useFileStore = defineStore('file', {
  state: (): State => ({
    files: []
  }),
  getters: {
    getFiles: (state: State) => state.files
  },
  actions: {
    addFile(file: string) {
      this.files?.push(file)
    },
    clearStore() {
      this.files = []
    }
  }
})
