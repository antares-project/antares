<script lang="ts">
	import type { Writable } from "svelte/store";
	import ServerIcon from "./serverIcon.svelte";

	const {
		onAddServer,
		servers,
		currentServer
	}: {
		onAddServer: () => void;
		servers: Writable<string[]>;
		currentServer: Writable<string | undefined>;
	} = $props();
</script>

<aside class="flex w-16 flex-col gap-2 border-r p-2">
	<div class="flex h-full flex-col gap-2">
		{#each $servers as publicKey}
			<ServerIcon
				onClick={() => {
					currentServer.set(publicKey);
				}}
				{publicKey}
			/>
		{/each}
	</div>
	<button onclick={onAddServer} class="h-12 w-12 cursor-pointer text-center">+</button>
</aside>
