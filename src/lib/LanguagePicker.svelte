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
		showList = false;
	}

	function handleClick() {
		showList = true;
	}
</script>

<div class="flex items-center space-x-2" on:blur={handleBlur} tabindex="-1">
		<label for="search" class="block text-white w-1/4">Translate to...</label>
		<div class="relative w-1/2">
			<input
				type="text"
				placeholder="Pick a target language"
				bind:value={search}
				class="bg-gray-800 w-full text-white p-2 rounded"
				on:focus={() => showList = true}
				on:click={handleClick}
			/>
			{#if showList}
				<div class="absolute flex flex-col w-full bg-gray-800 max-h-60 overflow-auto rounded-b-md z-50">
					{#each filteredLanguages as language}
						<button class="text-white text-left p-2 hover:bg-green-600 cursor-pointer" on:click={() => selectLanguage(language)}>{language}</button>
					{/each}
				</div>
			{/if}
		</div>
</div>