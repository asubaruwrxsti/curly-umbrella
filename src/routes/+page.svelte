<script>
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";

	let collection = { requests: [], workspace: "" };
	let result = "";
	let loading = false;
	let error = null;

	onMount(async () => {
		try {
			collection = await invoke("load_collection");
		} catch (err) {
			error = `Failed to load collection: ${err.message}`;
		}
	});

	async function sendRequest(req) {
		loading = true;
		error = null;
		try {
			result = await invoke("send_request", { req });
		} catch (err) {
			error = `Request failed: ${err.message}`;
		} finally {
			loading = false;
		}
	}
</script>

<h1>{collection.workspace}</h1>

{#if error}
	<div class="error">{error}</div>
{/if}

<ul>
	{#each collection.requests as req}
		<li>
			<button on:click={() => sendRequest(req)} disabled={loading}>
				{req.name} ({req.method})
			</button>
		</li>
	{/each}
</ul>

{#if loading}
	<div>Sending request...</div>
{/if}

<pre>{result}</pre>

<style>
	.error {
		color: red;
		margin: 1rem 0;
	}
</style>
