import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';
import autoprefixer from 'autoprefixer';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: preprocess({
		postcss: {
			plugins: [
				autoprefixer(),
			]
		}
	}),

	kit: {
		paths: { base: "" },
		target: '#svelte',
		adapter: adapter({
			// default options are shown
			pages: 'build',
			assets: 'build',
			fallback: null
		}),
		vite: {
			server: {
				fs: {
					allow: [
						'..'
					]
				}
			}
		}
	},
};

export default config;
