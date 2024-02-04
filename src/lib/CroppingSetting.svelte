<script lang="ts">
	let fromPercent = 0;
	let toPercent = 100;

	function validateFrom(event: FocusEvent) {
		const target = event.target as HTMLButtonElement;
		if (Number(target.value) > toPercent) {
			target.value = "0";
		} else {
			target.value = isNaN(Number(target.value)) ? "0" : String(Math.max(0, Math.min(100, Number(target.value))));
		}
	}

	function validateTo(event: FocusEvent) {
		const target = event.target as HTMLButtonElement;
		if (Number(target.value) < fromPercent) {
			target.value = "100";
		} else {
			target.value = isNaN(Number(target.value)) ? "100" : String(Math.max(0, Math.min(100, Number(target.value))));
		}
	}

</script>

<div class="flex items-center text-white bg-gray-800 space-x-3 p-3 rounded-md h-14">
		<label for="fromPercent" class="text-white w-1/4">Crop from...</label>
	<div class="w-1/2 flex items-center">
		<input
			id="fromPercent"
			type="text"
			bind:value={fromPercent}
			on:blur={validateFrom}
			class="rounded bg-gray-700 text-center w-12"
			placeholder="0"
		/>
		<label class= "me-3" for="fromPercent">%</label>
		<label class= "mx-3" for="toPercent">to</label>
		<input
			id="toPercent"
			type="text"
			bind:value={toPercent}
			on:blur={validateTo}
			class="ms-3 rounded bg-gray-700 text-center w-12"
			placeholder="100"
		/>
		<label class= "me-3" for="toPercent">%</label>
	</div>
</div>