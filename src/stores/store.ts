import { type Writable, writable } from 'svelte/store';

export const savedSubs: Writable<{original_text: string, translated_text: string,
	detected_source_language: string, timestamp: number}[]> = writable([]);