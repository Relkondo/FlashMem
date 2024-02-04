<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	let languages = ['English', 'French', 'Spanish', 'German', 'Italian'];
	let search = 'English';
	let showList = false;
	let dropdownElement: HTMLDivElement;

	$: filteredLanguages = languages.filter(lang =>
		lang.toLowerCase().includes(search.toLowerCase())
	);

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
	});

	onDestroy(() => {
		document.removeEventListener('click', handleClickOutside);
	});

	function selectLanguage(lang: string) {
		search = lang;
		showList = false;
	}

	function closeDropdown() {
		showList = false;
		if (!filteredLanguages.includes(search)) {
			search = 'English';
		}
	}

	function handleClickInside() {
		search = '';
		showList = true;
	}

	function handleClickOutside(event: MouseEvent) {
		if (!dropdownElement.contains(event.target as Node)) {
			closeDropdown();
		}
	}
</script>
<div bind:this={dropdownElement} class="flex items-center bg-gray-800 space-x-3 p-3 rounded-md h-14" on:blur={closeDropdown} tabindex="-1">
		<label for="search" class="text-white w-1/4">Translate to...</label>
		<div class="relative w-1/2">
			<input
				type="text"
				placeholder="Pick a target language..."
				bind:value={search}
				class="bg-gray-700 w-full text-white px-4 py-2 rounded"
				on:focus={() => showList = true}
				on:click={handleClickInside}
			/>
			{#if showList}
				<div class="absolute flex flex-col w-full bg-gray-700 max-h-60 overflow-auto rounded-b-md z-50">
					{#each filteredLanguages as language}
						<button class="text-white text-left p-2 hover:bg-green-600 cursor-pointer" on:click={() => selectLanguage(language)}>{language}</button>
					{/each}
				</div>
			{/if}
		</div>
</div>