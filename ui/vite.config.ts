import { defineConfig } from 'vite';
import path from 'node:path';

export default defineConfig({
  build: {
    lib: {
      entry: path.resolve(__dirname, 'src/main.tsx'),
      name: 'KivyJSXApp',
      fileName: () => 'app.bundle.js',
      formats: ['iife'],
    },
    rollupOptions: {
      external: [],
      output: {
        compact: true,
      },
    },
    target: 'esnext',
    outDir: 'dist',
    emptyOutDir: true,
  },
});
