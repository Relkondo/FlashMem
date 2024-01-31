<script>
	import { invoke } from '@tauri-apps/api/tauri'
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { onMount } from 'svelte';
	import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

	let activated = false;
	const TITLE = "FlashMem Translated Sub";

	onMount(async () => {
		await toggle_activation();
	});

	async function register_shortcut() {
		try {
			await register('Ctrl+G', execute);
			console.log('Shortcut Ctrl+G registered');
		} catch (error) {
			console.error('Error registering shortcut:', error);
		}
	}

	async function execute() {
		console.log('Ctrl+G pressed');
		let notification = await invoke('execute');
		if (notification !== "###-Already Running-###") {
			console.log('Sending notification...');
			await send_notification(TITLE, notification);
		} else {
			console.log('Cannot send notification, previous notification still in progress...');
		}
	}

	/** @param {string} title
	 *  @param {string} notification */
	async function send_notification(title, notification) {
		let permissionGranted = await isPermissionGranted();
		if (!permissionGranted) {
			const permission = await requestPermission();
			permissionGranted = permission === 'granted';
		}
		if (permissionGranted) {
			sendNotification({ title: title, body: notification });
		}
	}

	async function toggle_activation() {
		if (activated) {
			await unregisterAll();
		} else {
			await register_shortcut()
		}
		activated = !activated;
	}

</script>

<div>
	<button on:click="{toggle_activation}">{activated ? "Stop FlashMem" : "Start FlashMem"}</button>
</div>