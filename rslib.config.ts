import { defineConfig } from '@rslib/core'

export default defineConfig({
  source: {
    entry: {
      index: ['./src/**', '!src/lib.rs'],
    },
  },
  lib: [
    {
      format: 'esm',
      syntax: 'es2021',
      dts: true,
      bundle: false,
    },
  ],
})
