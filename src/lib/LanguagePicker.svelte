<script>
	let languages = ['English', 'French', 'Spanish', 'German', 'Italian'];
	let filteredLanguages = languages;
	let search = '';
	let showList = false;

	/** @param {string} lang */
	function selectLanguage(lang) {
		search = lang;
		showList = false;
	}

	function handleBlur() {
		setTimeout(() => { showList = false; }, 100);
	}

	function handleDoubleClick() {
		showList = true;
	}
</script>

<div class="relative" on:blur={handleBlur} tabindex="-1">
	<label for="search" class="block text-white mb-2">Translate to...</label>
	<input
		type="text"
		placeholder="Pick a target language"
		bind:value={search}
		class="w-full bg-gray-800 text-white p-2 rounded"
		on:focus={() => showList = true}
		on:dblclick={handleDoubleClick}
	/>
	{#if showList}
		<div class="absolute w-full bg-gray-800 max-h-60 overflow-auto rounded-b-md z-50">
			{#each filteredLanguages as language}
					<button class="text-white text-left p-2 hover:bg-green-600 cursor-pointer w-full" on:click={() => selectLanguage(language)}>{language}</button>
			{/each}
		</div>
	{/if}
</div>