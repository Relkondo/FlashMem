
<script lang="ts">
	import "../app.css";
	import { page } from '$app/stores';
	import { onDestroy, onMount, setContext } from 'svelte';
	import { savedSubs, shortcut } from '../stores/store.js';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
	import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

	const TITLE: string = 'FlashMem Translated Sub';

	onMount(async() => register_shortcut($shortcut));

	onDestroy(unregisterAll);

	setContext('register_shortcut', register_shortcut);

	async function register_shortcut(new_shortcut: string) {
		try {
			await unregisterAll();
			await register(new_shortcut, execute);
			shortcut.set(new_shortcut);
			console.log('Shortcut ' + new_shortcut + ' registered');
		} catch (error) {
			console.error('Error registering shortcut ' + new_shortcut + ' :', error);
		}
	}

	async function execute() {
		console.log(shortcut + ' pressed');
		let savedSub: {original_text: string, translated_text: string, detected_source_language: string, timestamp: number} = await invoke('execute');
		savedSub.timestamp = Date.now();
		if (savedSub.original_text === '###-Already Running-###') {
			console.log('Cannot send notification, previous notification still in progress...');
		} else {
			console.log('Sending notification...');
			if (savedSub.original_text.trim() === '') {
				await send_notification("FlashMem Error", 'No subtitles found!');
			} else {
				savedSubs.update(currentSubs => [
					...currentSubs,
					savedSub
				]);
				await send_notification(TITLE, format_notification(savedSub.translated_text, savedSub.detected_source_language));
			}
		}
	}

	function format_notification(translated_text: string, detected_source_language: string) {
		if (detected_source_language != "") {
			translated_text += `\n[Detected Source Language: ${detected_source_language}]`;
		}
		return translated_text;
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

</script>

<div class="flex flex-col align-top fixed top-0 w-full h-16 z-10 bg-gray-900">
	<nav class="w-fit h-12 py-2">
		<a href="/"
			 class="hover:text-green-500 py-4 px-5 text-center align-middle transition-colors duration-300 rounded leading-loose
			{$page.url.pathname === '/' ? 'bg-gray-800':''}">
			Home</a>
		<a href="/saved_subs"
			 class="hover:text-green-500 py-4 px-5 text-center align-middle transition-colors duration-300 rounded leading-loose
			{$page.url.pathname === '/saved_subs' ? 'bg-gray-800':''}">
			Saved Subs</a>
		<!--		<a href="/advanced_settings"-->
		<!--			 class="hover:text-green-500 py-4 px-5 text-center align-middle transition-colors duration-300 rounded leading-loose-->
		<!--				{$page.url.pathname === '/advanced_settings' ? 'bg-gray-800':''}">-->
		<!--			Advanced Settings</a>-->
	</nav>
</div>
<div class="px-4 pt-16 pd-4 overflow-auto">
	<slot />
</div>