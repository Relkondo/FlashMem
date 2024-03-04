<script lang="ts">
	import '../app.css';
	import SettingsPicker from '$lib/SettingsPicker.svelte';
	import { onMount, onDestroy } from 'svelte';
	import { register, unregisterAll } from '@tauri-apps/api/globalShortcut';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/shell';
	import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

	const TITLE: string = 'FlashMem Translated Sub';
	const APPLE_HELP_RECORDING_SCREEN_LINK: string = 'https://support.apple.com/guide/mac-help/control-access-screen-system-audio-recording-mchld6aa7d23/mac';
	const APPLE_HELP_NOTIFICATIONS_LINK: string = 'https://support.apple.com/fr-fr/guide/mac-help/mh40583/mac';
	let target_languages = ['English', 'French', 'Spanish', 'German', 'Italian', 'Portuguese', 'Korean', 'Japanese', 'Chinese', 'Vietnamese', 'Russian', 'Arabic', 'Hindi', 'Indonesian', 'Turkish'];
	let origin_languages = ['Automatic', 'English', 'French', 'Spanish', 'German', 'Italian', 'Portuguese', 'Korean', 'Japanese', 'Chinese', 'Vietnamese', 'Russian', 'Arabic', 'Hindi', 'Indonesian', 'Turkish'];
	let origin_language = 'Automatic';
	let target_language = 'English';
	let platforms = ['Default', 'Netflix', 'Amazon Prime Video', 'AppleTV', 'Hulu', 'Max', "YouTube", "VLC"]
	let shortcuts = ['Ctrl+T', 'Ctrl+Shift+T', 'Ctrl+Alt+T', 'Ctrl+X', 'Ctrl+Shift+X', 'Ctrl+Alt+X'];
	let current_shortcut = 'Ctrl+T';
	let showHelpLink = false;

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

	function handleOriginLanguageSelected(event: CustomEvent) {
		origin_language = event.detail.value;
		if (origin_language == target_language) {
			target_language = origin_language == "English"? "Spanish" : "English";
		}
	}

	function handleTargetLanguageSelected(event: CustomEvent) {
		target_language = event.detail.value;
		if (origin_language == target_language) {
			origin_language = "Automatic";
		}
	}


	function handleShortcutSelected(event: CustomEvent) {
		register_shortcut(event.detail.value);
		console.log('Shortcut selected:', event.detail.value);
	}

	function toggleHelpLink() {
		showHelpLink = !showHelpLink;
	}

	async function openRecordingScreenHelpLink() {
		await open(APPLE_HELP_RECORDING_SCREEN_LINK);
	}

	async function openNotificationsHelpLink() {
		await open(APPLE_HELP_NOTIFICATIONS_LINK);
	}

</script>

<style>
    .link-button {
        background: none;
        border: none;
        padding: 0;
        text-decoration: underline;
        cursor: pointer;
    }
		.link-button:hover {
        color: rgb(34 197 94);
    }
</style>

<div class="min-h-screen bg-gray-900 flex flex-col items-center">
	<div class="text-white bg-gray-800 p-4 rounded shadow-lg max-w-4xl mx-auto my-8">
		<h1 class="text-xl text-center font-bold mb-2">Welcome to FlashMem!</h1>
		<p>Automatically translate and save movies subs. Just press a shortcut while watching!</p>
	</div>
	<div class="w-full space-y-3 max-w-md">
		<SettingsPicker items={origin_languages} label="Translate from..." placeholder="Pick an origin language..."
										value={origin_language} command="set_origin_language" on:valueSelected={handleOriginLanguageSelected} />
		<SettingsPicker items={target_languages} label="Translate to..." placeholder="Pick a target language..."
										value={target_language} command="set_target_language" on:valueSelected={handleTargetLanguageSelected} />
		<SettingsPicker items={platforms} label="Optimize for..." placeholder="Pick a platform..."
										value="Default" command="set_platform"/>
		<SettingsPicker items={shortcuts} label="Shortcut to press..." placeholder="Pick a shortcut..."
										value="Ctrl+T" on:valueSelected={handleShortcutSelected} />
	</div>

	<div class="text-white bg-gray-800 p-4 rounded shadow-lg max-w-4xl mx-auto my-8">
	{#if showHelpLink}
		<p class="text-center"><button class="link-button" on:click={toggleHelpLink}>Got it!</button></p>
		<br/>
		<li class="mb-2">Make sure you allowed FlashMem to <button class="link-button" on:click={openRecordingScreenHelpLink}>record your screen</button></li>
		<li class="mb-2">Make sure you allowed FlashMem to <button class="link-button" on:click={openNotificationsHelpLink}>send you notifications</button></li>
		<li class="mb-2">Make sure you watch the movie fullscreen!</li>
	{:else}
		<p class="text-center">It's not working, <button class="link-button" on:click={toggleHelpLink}>help!</button></p>
	{/if}
	</div>
</div>
