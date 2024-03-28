<script lang="ts">
	import '../app.css';
	import SettingsPicker from '$lib/SettingsPicker.svelte';
	import { open } from '@tauri-apps/api/shell';
	import { origin_language, target_language, platform, shortcut } from '../stores/store';
	import { getContext } from 'svelte';
	import { initializeApp } from "firebase/app";

	const APPLE_HELP_RECORDING_SCREEN_LINK: string = 'https://support.apple.com/guide/mac-help/control-access-screen-system-audio-recording-mchld6aa7d23/mac';
	const APPLE_HELP_NOTIFICATIONS_LINK: string = 'https://support.apple.com/fr-fr/guide/mac-help/mh40583/mac';
	let target_languages = ['English', 'French', 'Spanish', 'German', 'Italian', 'Portuguese', 'Korean', 'Japanese', 'Chinese', 'Vietnamese', 'Russian', 'Arabic', 'Hindi', 'Indonesian', 'Turkish'];
	let origin_languages = ['Automatic', 'English', 'French', 'Spanish', 'German', 'Italian', 'Portuguese', 'Korean', 'Japanese', 'Chinese', 'Vietnamese', 'Russian', 'Arabic', 'Hindi', 'Indonesian', 'Turkish'];
	let platforms = ['Default', 'Netflix', 'Amazon Prime Video', 'AppleTV', 'Hulu', 'Max', "YouTube", "VLC"]
	let shortcuts = ['Ctrl+T', 'Ctrl+Shift+T', 'Ctrl+Alt+T', 'Ctrl+X', 'Ctrl+Shift+X', 'Ctrl+Alt+X'];
	let showHelpLink = false;
	const register_shortcut: (shortcut: string) => Promise<void> = getContext('register_shortcut');
	const firebaseConfig = {
		apiKey: "AIzaSyBCgq-qsmF6A4LCsWDO9YgAOgVcFXaDXNw",
		authDomain: "flashsub-1b456.firebaseapp.com",
		projectId: "flashsub-1b456",
		storageBucket: "flashsub-1b456.appspot.com",
		messagingSenderId: "368028589856",
		appId: "1:368028589856:web:d3e015ef2a6e9b78b40145",
		measurementId: "G-82XNXVVPS9"
	};
	initializeApp(firebaseConfig);



	function handleOriginLanguageSelected(event: CustomEvent) {
		origin_language.set(event.detail.value);
		if ($origin_language == $target_language) {
			target_language.update(() => $origin_language == "English" ? "Spanish" : "English");
		}
	}

	function handleTargetLanguageSelected(event: CustomEvent) {
		target_language.set(event.detail.value);
		if ($origin_language == $target_language) {
			origin_language.set("Automatic");
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

<div class="flex flex-col items-center pt-4">
	<div class="text-white bg-gray-800 p-4 rounded shadow-lg max-w-4xl mx-auto mb-8">
		<h1 class="text-xl text-center font-bold mb-2">Welcome to FlashSub!</h1>
		<p>Automatically translate and save movies subs. Just press a shortcut while watching!</p>
	</div>
	<div class="w-full space-y-3 max-w-md">
		<SettingsPicker items={origin_languages} label="Translate from..." placeholder="Pick an origin language..."
										setting={origin_language} command="set_origin_language" on:valueSelected={handleOriginLanguageSelected} />
		<SettingsPicker items={target_languages} label="Translate to..." placeholder="Pick a target language..."
										setting={target_language} command="set_target_language" on:valueSelected={handleTargetLanguageSelected} />
		<SettingsPicker items={platforms} label="Optimize for..." placeholder="Pick a platform..."
										setting={platform} command="set_platform"/>
		<SettingsPicker items={shortcuts} label="Shortcut to press..." placeholder="Pick a shortcut..."
										setting={shortcut} on:valueSelected={handleShortcutSelected} />
	</div>

	<div class="text-white bg-gray-800 p-4 rounded shadow-lg max-w-4xl mx-auto my-8">
	{#if showHelpLink}
		<p class="text-center"><button class="link-button" on:click={toggleHelpLink}>Got it!</button></p>
		<br/>
		<li class="mb-2">Make sure you allowed FlashSub to <button class="link-button" on:click={openRecordingScreenHelpLink}>record your screen</button></li>
		<li class="mb-2">Make sure you allowed FlashSub to <button class="link-button" on:click={openNotificationsHelpLink}>send you notifications</button></li>
		<li class="mb-2">Make sure you watch the movie fullscreen!</li>
	{:else}
		<p class="text-center">It's not working, <button class="link-button" on:click={toggleHelpLink}>help!</button></p>
	{/if}
	</div>
</div>
