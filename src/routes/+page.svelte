<script>
	import Execute from '$lib/Execute.svelte';
	import "../app.css";
	import LanguagePicker from '$lib/LanguagePicker.svelte';
	import CroppingSetting from '$lib/CroppingSetting.svelte';
</script>

<div class="min-h-screen bg-gray-900 p-8">
	<div class="container mx-auto text-white">
		<h1 class="text-2xl font-bold mb-4">App Settings</h1>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
			<div>
				<h2 class="text-xl font-semibold mb-2"></h2>
				<Execute />
			</div>
			<div>
				<h2 class="text-xl font-semibold mb-2">Language Picker</h2>
				<LanguagePicker />
			</div>
			<div>
				<h2 class="text-xl font-semibold mb-2">Cropping Setting</h2>
				<CroppingSetting />
			</div>
		</div>
	</div>
</div>