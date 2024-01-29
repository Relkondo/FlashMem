<script>
	import { invoke } from '@tauri-apps/api/tauri'

	let running = false;

	async function handle_click() {
		if (running) {
			await invoke('stop');
		} else {
			await invoke('start');
		}
		running = !running;
	}

</script>

<div>
	<button on:click="{handle_click}">{running ? "Stop FlashMem" : "Start FlashMem"}</button>
</div>