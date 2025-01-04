import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

const DEV_API_URL = process.env.API_URL || 'http://localhost:8080';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: DEV_API_URL,
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	}
});
