<script lang="ts">
	import { getAuth, signInWithEmailAndPassword,createUserWithEmailAndPassword } from 'firebase/auth';
	let auth = getAuth();
	import { onMount } from 'svelte';
	import { signOut } from 'firebase/auth';
	import { user } from '../../stores/store';

	let email = '';
	let password = '';
	let isSignUp = true;
	let errorMessage = '';

	async function handleSubmit() {
		try {
			if (isSignUp) {
				await signInWithEmailAndPassword(auth, email, password);
			} else {
				await createUserWithEmailAndPassword(auth, email, password);
			}
		} catch (error) {
			errorMessage = (error as Error).message;
			console.log(errorMessage);
		}
	}

	async function handleLogout() {
		await signOut(auth);
		// Optionally, delete local files or data here
	}

	onMount(() => {
		// Optionally, check if user is already signed in and redirect
	});
</script>

<div class="flex flex-col items-center pt-4 w-full">
	{#if $user !== ""}
		<p>Email: {$user}</p>
		<button on:click={handleLogout}
						class="border-2 border-cyan-900 rounded-md p-2">Log Out</button>
	{:else}
		<form on:submit|preventDefault={handleSubmit}
					class="mt-3 flex flex-col bg-gray-800 items-center w-full max-w-lg space-y-4 pt-5 pb-4 rounded-md">
			<input type="email"
						 bind:value={email}
						 placeholder="Email"
						 class="bg-gray-700 text-white text-center w-5/6 h-9 rounded"
						 required>
			<input type="password"
						 bind:value={password}
						 placeholder="Password"
						 class="bg-gray-700 text-white text-center w-5/6 h-9 rounded"
						 required>
			<button type="submit"
							class="border-2 border-cyan-900 rounded-md p-2 text-lg"
			>{isSignUp ? 'Create an Account' : 'Log In'}</button>
			<button on:click={() => isSignUp = !isSignUp}
							class="border-2 border-cyan-900 rounded-md p-2 text-sm"
			>{isSignUp ? 'I already have one':'I don\'t have an account yet'}</button>
			{#if errorMessage}
				<p>{errorMessage}</p>
			{/if}
			</form>
	{/if}
</div>

