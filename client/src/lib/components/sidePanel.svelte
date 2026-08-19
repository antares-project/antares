<script lang="ts">
	import { type ServerData } from "$lib/server.svelte";
	import type { Writable } from "svelte/store";

	const {
		onAddServer,
		servers,
		currentServer
	}: {
		onAddServer: () => void;
		servers: Writable<ServerData[]>;
		currentServer: Writable<string | undefined>;
	} = $props();
</script>

{#snippet serverIcon(data: ServerData, onClick: () => void)}
	<button
		onclick={onClick}
		class="group relative flex cursor-pointer flex-col items-center justify-center"
	>
		<img src={`${data.url}/icon`} alt="icon" class="h-12 w-12 rounded-xl" />
		<div
			class="pointer-events-none absolute left-full ml-2 flex translate-x-2 items-center justify-center rounded-md bg-slate-700 p-1 opacity-0 transition-all duration-200 group-hover:translate-x-0 group-hover:opacity-100"
		>
			<p>{data.title}</p>
		</div>
	</button>
{/snippet}

<aside class="flex w-16 flex-col gap-2 border-r p-2">
	<div class="flex h-full flex-col gap-2">
		{#each $servers as data}
			{@render serverIcon(data, () => {
				currentServer.set(data.publicKey);
			})}
		{/each}
	</div>
	<button onclick={onAddServer} class="h-12 w-12 cursor-pointer text-center">+</button>
</aside>
