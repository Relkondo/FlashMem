<script>
	import "../app.css";
	import SettingsPicker from '$lib/SettingsPicker.svelte';
	import CroppingSetting from '$lib/CroppingSetting.svelte';
	import { onMount, onDestroy } from 'svelte';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
	import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

	const TITLE = "FlashMem Translated Sub";

	let languages = ['English', 'French', 'Spanish', 'German', 'Italian'];

	onMount(async () => {
		await register_shortcut();
	});

	onDestroy(async () => {
		await unregisterAll();
	});

	async function register_shortcut() {
		try {
			await unregisterAll();
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

</script>

<div class="min-h-screen bg-gray-900 p-8 flex flex-col items-center">
			<div class="w-full space-y-3 max-w-md">
					<SettingsPicker items={languages} label="Translate to..." placeholder="Pick a target language..." defaultPick="English"/>
					<CroppingSetting />
			</div>
</div>
