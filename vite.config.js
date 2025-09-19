import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';
import pkg from './package.json' assert { type: 'json' };

export default defineConfig(({ mode }) => {
  const isProd = mode === 'production';
  return {
    plugins: [vue()],
    resolve: {
      alias: {
        '@': path.resolve(__dirname, 'src'),
        '@components': path.resolve(__dirname, 'src/components'),
        '@assets': path.resolve(__dirname, 'src/assets'),
        '@interfaces': path.resolve(__dirname, 'src/interfaces'),
        '@helpers': path.resolve(__dirname, 'src/helpers'),
        '@common': path.resolve(__dirname, 'src/common'),
        '@composables': path.resolve(__dirname, 'src/composables'),
      },
    },
    define: {
      __APP_VERSION__: JSON.stringify(pkg.version),
    },
    build: {
      sourcemap: false,
      minify: 'esbuild',
      esbuild: isProd
        ? {
            drop: ['console', 'debugger'],
            pure: ['console.log', 'console.info', 'console.debug'],
          }
        : undefined,
      rollupOptions: {
        output: {
          manualChunks: {
            vendor: ['vue'],
            tauri: ['@tauri-apps/api'],
          },
        },
      },
    },
  };
});
