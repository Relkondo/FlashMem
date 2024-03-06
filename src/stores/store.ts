import { type Writable, writable } from 'svelte/store';

export const savedSubs: Writable<{original_text: string, translated_text: string,
	detected_source_language: string, timestamp: number}[]> = writable([]);

export const origin_language: Writable<string> = writable("Automatic");
export const target_language: Writable<string> = writable("English");
export const platform: Writable<string> = writable("Default");
export const shortcut: Writable<string> = writable("Ctrl+T");