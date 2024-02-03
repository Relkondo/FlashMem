<script>
	let languages = ['English', 'French', 'Spanish', 'German', 'Italian'];
	let filteredLanguages = languages;
	let search = '';
	let isFocused = false; // Local state to track focus

	$: filteredLanguages = languages.filter((lang) =>
		lang.toLowerCase().includes(search.toLowerCase())
	);
</script>

<div class="relative" on:blur={() => isFocused = false} tabindex="-1">
	<input
		type="text"
		placeholder="Search language..."
		bind:value={search}
		class="w-full bg-gray-800 text-white p-2 rounded-t-md focus:rounded-b-none"
		on:focus={() => isFocused = true}
	/>
	{#if isFocused}
		<div class="absolute w-full bg-gray-800 max-h-60 overflow-auto rounded-b-md">
			{#each filteredLanguages as language}
				<div class="text-white p-2 hover:bg-green-600 cursor-pointer">
					{language}
				</div>
			{/each}
		</div>
	{/if}
</div>