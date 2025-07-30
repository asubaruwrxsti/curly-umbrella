<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api';

  let collection = { requests: [] };
  let result = '';

  onMount(async () => {
    collection = await invoke('load_collection');
  });

  async function sendRequest(req) {
    result = await invoke('send_request', { req });
  }
</script>

<h1>{collection.workspace}</h1>
<ul>
  {#each collection.requests as req}
    <li>
      <button on:click={() => sendRequest(req)}>
        {req.name} ({req.method})
      </button>
    </li>
  {/each}
</ul>

<pre>{result}</pre>
