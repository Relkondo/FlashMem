<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { register } from '@tauri-apps/api/globalShortcut';
	import { onMount } from 'svelte';

	let running = false;

	onMount(async () => {
		console.log('Registering shortcut...');
		try {
			await register('Ctrl+G', () => {
				console.log('Ctrl+G pressed');
				invoke('execute');
			});
			console.log('Shortcut Ctrl+G registered');
		} catch (error) {
			console.error('Error registering shortcut:', error);
		}
	});

	async function handle_click() {
		if (running) {
			await invoke('deactivate');
		} else {
			await invoke('activate');
		}
		running = !running;
	}

</script>

<div>
	<button on:click="{handle_click}">{running ? "Stop FlashMem" : "Start FlashMem"}</button>
</div>