<script lang="ts">
	import { greet } from '$lib/api/greetings';

	let name = '';
	let greeting = '';
	let loading = false;

	async function handleGreet() {
		if (!name.trim()) {
			greeting = 'Please enter a name';
			return;
		}

		loading = true;
		try {
			greeting = await greet(name);
		} catch (error) {
			greeting = `Error: ${error}`;
		} finally {
			loading = false;
		}
	}
</script>

<main>
	<div class="container">
		<h1>Hello World</h1>
		<p>Welcome to RUSTALK!</p>

		<div class="greeting-form">
			<input
				type="text"
				bind:value={name}
				placeholder="Enter your name"
				disabled={loading}
			/>
			<button on:click={handleGreet} disabled={loading || !name.trim()}>
				{loading ? 'Greeting...' : 'Greet Me'}
			</button>
		</div>

		{#if greeting}
			<div class="greeting-result">
				<p>{greeting}</p>
			</div>
		{/if}
	</div>
</main>

<style>
	.container {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		padding: 2rem;
		font-family: system-ui, -apple-system, sans-serif;
	}

	h1 {
		font-size: 2.5rem;
		margin-bottom: 0.5rem;
		color: #333;
	}

	p {
		color: #666;
		margin-bottom: 2rem;
	}

	.greeting-form {
		display: flex;
		gap: 1rem;
		margin-bottom: 2rem;
	}

	input {
		padding: 0.75rem 1rem;
		font-size: 1rem;
		border: 2px solid #ddd;
		border-radius: 4px;
		min-width: 200px;
	}

	input:focus {
		outline: none;
		border-color: #007bff;
	}

	input:disabled {
		background-color: #f5f5f5;
		cursor: not-allowed;
	}

	button {
		padding: 0.75rem 1.5rem;
		font-size: 1rem;
		background-color: #007bff;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: background-color 0.2s;
	}

	button:hover:not(:disabled) {
		background-color: #0056b3;
	}

	button:disabled {
		background-color: #ccc;
		cursor: not-allowed;
	}

	.greeting-result {
		padding: 1rem;
		background-color: #f0f0f0;
		border-radius: 4px;
		min-width: 300px;
		text-align: center;
	}

	.greeting-result p {
		margin: 0;
		font-size: 1.1rem;
		color: #333;
	}
</style>

