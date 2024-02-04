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

<div class="text-white flex bg-gray-800 space-x-2 p-3 rounded-md">
	<div>
		<label for="fromPercent">Crop from:</label>
		<input
			id="fromPercent"
			type="text"
			bind:value={fromPercent}
			on:blur={validateFrom}
			class="p-2 rounded bg-gray-700 text-white w-1/2 md:w-1/3"
			placeholder="0"
		/>
		<label for="fromPercent">%</label>
	</div>
	<div>
		<label for="toPercent">to:</label>
		<input
			id="toPercent"
			type="text"
			bind:value={toPercent}
			on:blur={validateTo}
			class="p-2 rounded bg-gray-700 text-white w-1/2 md:w-1/3"
			placeholder="100"
		/>
		<label for="toPercent">%</label>
	</div>
</div>