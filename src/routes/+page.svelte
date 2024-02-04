<script>
	import Execute from '$lib/Execute.svelte';
	import "../app.css";
	import LanguagePicker from '$lib/LanguagePicker.svelte';
	import CroppingSetting from '$lib/CroppingSetting.svelte';
</script>

<div class="min-h-screen bg-gray-900 p-8 flex flex-col items-center">
			<div class="mb-8 p-4">
				<Execute />
			</div>
			<div class="w-full space-y-3 max-w-md">
					<LanguagePicker />
					<CroppingSetting />
			</div>
</div>
