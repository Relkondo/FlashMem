<script>
	import '../app.css';
	import SettingsPicker from '$lib/SettingsPicker.svelte';
	import { onMount, onDestroy } from 'svelte';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
	import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

	const TITLE = 'FlashMem Translated Sub';

	let languages = ['English', 'French', 'Spanish', 'German', 'Italian'];
	let platforms = ['Default', 'Netflix', 'Youtube', 'Amazon Prime Video', 'AppleTV', 'Hulu'];
	let shortcuts = ['Ctrl+T', 'Ctrl+Shift+T', 'Ctrl+Alt+T'];

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
		if (notification !== '###-Already Running-###') {
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
	<div class="text-white bg-gray-800 p-4 rounded shadow-lg max-w-4xl mx-auto my-8">
		<h1 class="text-xl text-center font-bold mb-2">Welcome to FlashMem!</h1>
		<p>Automatically translate and save subs when watching movies with just a shortcut.</p>
		<p>Just make sure you watch the movie fullscreen!</p>
	</div>
	<div class="w-full space-y-3 max-w-md">
		<SettingsPicker items={languages} label="Translate to..." placeholder="Pick a target language..."
										defaultPick="English" />
		<SettingsPicker items={platforms} label="Optimize for..." placeholder="Pick a platform..."
										defaultPick="Default" />
		<SettingsPicker items={shortcuts} label="Shortcut to press..." placeholder="Pick a shortcut..."
										defaultPick="Ctrl+T" />
	</div>
</div>
