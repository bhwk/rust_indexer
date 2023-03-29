<script lang="ts">
	import { invoke } from '@tauri-apps/api';

	let dirPath = '/root/sideprojects/rust_indexer/';
	let term: String;
	let searchResults: any;

	async function build_index() {
		await invoke('build_index', { dirPath }).then(() => console.log('Indexing complete'));
	}

	async function search_files(term: String) {
		let results;
		await invoke('search_files', { term: term })
			.then((data) => {
				console.log(data);
				results = data;
			})
			.catch((err) => console.log(err));
		return results;
	}
</script>

<button on:click={build_index}>Build index</button>
<button
	on:click={async () => {
		searchResults = await search_files(term);
	}}>Search</button
>
<input bind:value={term} />
{#if searchResults}
	{#each Object.entries(searchResults) as [path, filename]}
		<div>{filename}: {path}</div>
	{/each}
{/if}
