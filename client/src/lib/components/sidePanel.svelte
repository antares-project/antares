<script lang="ts">
    import { type ServerData } from "$lib/server.svelte";
    import type { Writable } from "svelte/store";

    const {
        onAddServer,
        servers,
        currentServer,
    }: {
        onAddServer: () => void;
        servers: Writable<ServerData[]>;
        currentServer: Writable<ServerData | undefined>;
    } = $props();
</script>

{#snippet serverIcon(onClick: () => void, data: ServerData)}
    <button onclick={onClick} class="relative flex flex-col items-center justify-center group cursor-pointer">
        <img src={`${data.url}/icon`} alt="icon" class="h-12 w-12 rounded-xl" />
        <div
            class="absolute bg-slate-700 p-1 rounded-md items-center justify-center flex left-full ml-2 opacity-0 translate-x-2 transition-all duration-200 group-hover:opacity-100 group-hover:translate-x-0 pointer-events-none"
        >
            <p>{data.title}</p>
        </div>
    </button>
{/snippet}

<aside class="flex flex-col w-16 gap-2 border-r p-2">
    <div class="flex flex-col h-full gap-2">
        {#each $servers as data}
            {@render serverIcon(() => {
                currentServer.set(data);
            }, data)}
        {/each}
    </div>
    <button onclick={onAddServer} class="h-12 w-12 text-center cursor-pointer">+</button>
</aside>
