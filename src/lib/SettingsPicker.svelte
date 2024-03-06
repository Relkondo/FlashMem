<script lang="ts">
	import { onMount, onDestroy, createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { writable } from 'svelte/store';
	const dispatch = createEventDispatcher();
	export let items: string[] = [];
	export let label = '';
	export let placeholder = '';
	export let setting = writable('');
	export let command= '';
	let showList = false;
	let dropdownElement: HTMLDivElement;
	let displayedValue = $setting;

	$: filteredItems = items.filter(i =>
		i.toLowerCase().includes(displayedValue.toLowerCase())
	);

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		setting.subscribe((val) => {
			displayedValue = val;
			if (command !== '') {
				invoke(command, { value: val })
					.then(() => console.log(`${command}: ${val}`))
					.catch((err) => console.error(`Failed to ${command}: ${err}`));
			}
		});
	});

	onDestroy(() => {
		document.removeEventListener('click', handleClickOutside);
	});

	function selectItem(item: string) {
		setting.set(item);
		showList = false;
		dispatch('valueSelected', { value: item });
	}

	function closeDropdown() {
		showList = false;
		if (!filteredItems.includes(displayedValue)) {
			displayedValue = $setting;
		}
	}

	function handleClickInside() {
		displayedValue = '';
		showList = true;
	}

	function handleClickOutside(event: MouseEvent) {
		if (!dropdownElement.contains(event.target as Node)) {
			closeDropdown();
		}
	}
</script>

<div bind:this={dropdownElement} class="flex items-center bg-gray-800 space-x-3 p-3 rounded-md h-14" on:blur={closeDropdown} tabindex="-1">
		<label for="value" class="text-white w-1/3">{label}</label>
		<div class="relative w-1/2">
			<input
				type="text"
				placeholder={placeholder}
				bind:value={displayedValue}
				class="bg-gray-700 w-full text-white px-4 py-2 rounded"
				on:focus={() => showList = true}
				on:click={handleClickInside}
			/>
			{#if showList}
				<div class="absolute flex flex-col w-full bg-gray-700 max-h-60 overflow-auto rounded-b-md z-50">
					{#each filteredItems as item}
						<button class="text-white text-left p-2 hover:bg-green-700 transition-colors cursor-pointer" on:click={() => selectItem(item)}>{item}</button>
					{/each}
				</div>
			{/if}
		</div>
</div>