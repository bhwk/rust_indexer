<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { appDataDir } from '@tauri-apps/api/path';

	let term: string;
	let searchResults: any;

	function isBlank(str: string) {
		return !str || /^\s*$/.test(str);
	}

	async function build_index() {
		await invoke('build_index').then(() => console.log('Indexing complete'));
	}

	async function open_file(path: string) {
		await invoke('open_file', { path: path })
			.then(() => console.log('File opened successfully'))
			.catch((err) => console.log(err));
	}

	async function search_files(term: string) {
		if (isBlank(term)) return;
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

<button
	on:click={async () => {
		let dataDir = await appDataDir();
		open_file(dataDir);
	}}>open data dir</button
>
<button on:click={build_index}>Build index</button>

<input
	bind:value={term}
	on:keydown={async (keypress) => {
		if (keypress.key == 'Enter') {
			searchResults = await search_files(term);
		}
	}}
/>
{#if searchResults}
	{#each Object.entries(searchResults) as [path, filename]}
		<div>
			<div>{filename}: {path}</div>
			<button
				on:click={() => {
					open_file(path);
				}}>Open file</button
			>
		</div>
	{/each}
{/if}
