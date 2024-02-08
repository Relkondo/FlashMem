<script lang="ts">
	import '../app.css';
	import SettingsPicker from '$lib/SettingsPicker.svelte';
	import { onMount, onDestroy } from 'svelte';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
	import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

	const TITLE: string = 'FlashMem Translated Sub';

	let languages = ['English', 'French', 'Spanish', 'German', 'Italian'];
	let platforms = ['Default', 'Netflix', 'Amazon Prime Video', 'AppleTV', 'Hulu', 'Max', "YouTube", "VLC"]
	let shortcuts = ['Ctrl+T', 'Ctrl+Shift+T', 'Ctrl+Alt+T', 'Ctrl+X', 'Ctrl+Shift+X', 'Ctrl+Alt+X'];
	let current_shortcut = 'Ctrl+T';

	onMount(async () => {
		await register_shortcut('Ctrl+T');
	});

	onDestroy(async () => {
		await unregisterAll();
	});

	async function register_shortcut(shortcut: string) {
		try {
			await unregisterAll();
			await register(shortcut, execute);
			current_shortcut = shortcut;
			console.log('Shortcut ' + shortcut + ' registered');
		} catch (error) {
			console.error('Error registering shortcut ' + shortcut + ' :', error);
		}
	}

	async function execute() {
		console.log(current_shortcut + ' pressed');
		let notification: string = await invoke('execute');
		if (notification !== '###-Already Running-###') {
			console.log('Sending notification...');
			await send_notification(TITLE, notification);
		} else {
			console.log('Cannot send notification, previous notification still in progress...');
		}
	}

	async function send_notification(title: string, notification: string) {
		let permissionGranted = await isPermissionGranted();
		if (!permissionGranted) {
			const permission = await requestPermission();
			permissionGranted = permission === 'granted';
		}
		if (permissionGranted) {
			sendNotification({ title: title, body: notification });
		}
	}

	function handleShortcutSelected(event: CustomEvent) {
		register_shortcut(event.detail.shortcut);
		console.log('Shortcut selected:', event.detail.shortcut);
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
										defaultPick="English" command="set_target_language" />
		<SettingsPicker items={platforms} label="Optimize for..." placeholder="Pick a platform..."
										defaultPick="Default" command="set_platform"/>
		<SettingsPicker items={shortcuts} label="Shortcut to press..." placeholder="Pick a shortcut..."
										defaultPick="Ctrl+T" command="set_shortcut" on:shortcutSelected={handleShortcutSelected} />
	</div>
</div>
