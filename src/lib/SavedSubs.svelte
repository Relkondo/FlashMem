<script lang="ts">
	import { savedSubs } from '../stores/store';

	function formatDate(date: number) {
		return new Date(date).toLocaleString();
	}

	function deleteSub(original_text: string, timestamp: number) {
		savedSubs.update(subs => subs.filter(sub => sub.timestamp !== timestamp || sub.original_text !== original_text));
	}
</script>

<div class="overflow-auto h-screen p-4">
	<div class="flex flex-col-reverse space-y-4 space-y-reverse">
		{#each $savedSubs as { original_text, translated_text, detected_source_language, timestamp }}
			<div class=" relative bg-gray-800 p-4 rounded-lg">
				<button
					class="absolute top-0 right-0 text-white text-lg p-2"
					on:click={() => deleteSub(original_text, timestamp)}
				><span class="text-sm text-gray-400">x</span></button>
				<p class="text-green-400 text-sm pb-1">{original_text}</p>
				<p class="text-gray-300 text-sm">{translated_text}</p>
				{#if detected_source_language !== ''}
					<p class="text-gray-400 text-xs">Detected source language: {detected_source_language}</p>
				{/if}
				<p class="text-gray-400 text-xs">Saved on: {formatDate(timestamp)}</p>
			</div>
		{/each}
	</div>
</div>