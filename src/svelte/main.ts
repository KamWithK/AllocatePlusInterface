import App from './app.svelte';

const app = new App({
	target: document.body,
	props: {
		name: 'allocate-plus-plus'
	}
});

export default app;