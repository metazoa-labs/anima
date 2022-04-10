import App from './App.svelte';
import './style/anima_theme.less';

const app = new App({
	target: document.body,
	props: {
		name: 'tauri',
	},
});

export default app;
