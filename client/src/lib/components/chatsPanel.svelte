<script lang="ts">
    import type { ServerData } from "$lib/server.svelte";
    import type { Channel, Client } from "antares-lib";
    import type { Writable } from "svelte/store";
    import { useAuth } from "$lib/auth";
    import { push } from "./toast.svelte";

    const {
        client,
        channelList,
        currentChannel,
        currentServer,
    }: {
        client: Client;
        channelList: Channel[];
        currentChannel: Writable<Channel | undefined>;
        currentServer: Writable<ServerData | undefined>;
    } = $props();

    const auth = useAuth();
    const pubKey = auth?.publicKey.toHex() ?? "";

    async function selectChannel(channel: Channel) {
        const current = await client.joinChannel(channel.id);

        currentChannel.set(current);
    }
</script>

<aside class="flex w-60 flex-col border-r">
    <div class="flex flex-col p-4 gap-2 justify-center items-center">
        <h1 class="text-center">{$currentServer?.title}</h1>
        <h3 class="text-center text-sm">{$currentChannel?.name}</h3>
    </div>
    <hr />
    <div class="flex grow flex-col text-gray-400 gap-2 p-2">
        {#each channelList as channel}
            <button
                onclick={() => selectChannel(channel)}
                class={`cursor-pointer flex items-center p-2 rounded-md hover:bg-gray-800 ${$currentChannel?.id === channel.id ? "bg-gray-800" : ""}`}
            >
                <span># {channel.name}</span>
            </button>
        {/each}
    </div>
    <hr />
    <div class="flex flex-col p-4 h-20 justify-center">
        <button
            class="truncate cursor-pointer"
            onclick={() => {
                navigator.clipboard.writeText(pubKey);
                push("Copied to clipboard");
            }}>{pubKey}</button
        >
    </div>
</aside>
