// @ts-check
import { defineConfig } from 'astro/config';
import tailwindcss from '@tailwindcss/vite';
import { loadEnv } from 'vite';

// @ts-ignore - process disediakan oleh Node.js runtime saat membaca .env
const env = loadEnv(process.env.NODE_ENV ?? 'development', process.cwd(), '');
const API_PROXY_TARGET = env.API_PROXY_TARGET || 'http://localhost:3000';

// https://astro.build/config
export default defineConfig({
  vite: {
    plugins: [tailwindcss()],
    server: {
      proxy: {
        '/api': {
          target: API_PROXY_TARGET,
          changeOrigin: true,
        },
      },
    },
  },
});
