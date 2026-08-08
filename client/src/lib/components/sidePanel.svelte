<script lang="ts">
    import { type ServerData } from "$lib/server.svelte";
    import type { Writable } from "svelte/store";
    import ServerIcon from "./serverIcon.svelte";

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

<aside class="flex flex-col w-16 gap-2 border-r p-2">
    <div class="flex flex-col h-full gap-2">
        {#each $servers as data}
            <ServerIcon
                {data}
                onClick={() => {
                    currentServer.set(data);
                }}
            />
        {/each}
    </div>
    <button onclick={onAddServer} class="h-12 w-12 text-center cursor-pointer"
        >+</button
    >
</aside>
