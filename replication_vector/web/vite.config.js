import { defineConfig } from 'vite'
import path from 'path'

export default defineConfig({
  server: {
    fs: {
      allow: [
        path.resolve(__dirname, '../pkg')
      ]
    }
  },
  resolve: {
    alias: {
      '@pkg': path.resolve(__dirname, '../pkg')
    }
  }
})
