<script lang="ts">
    import type { ServerData } from "$lib/server.svelte";
    import type { Channel, Client } from "harmon-lib";
    import type { Writable } from "svelte/store";
    import { useAuth } from "$lib/auth";
    import { push } from "./toast.svelte";
    import Fa from "svelte-fa";
    import { faHashtag, faVolume } from "@fortawesome/free-solid-svg-icons";
    import { uint8ArrayToZ32 } from "harmon-lib/utils";

    const {
        client,
        channelList,
        currentServer,
    }: {
        client: Client;
        channelList: Channel[];
        currentServer: Writable<ServerData | undefined>;
    } = $props();

    const auth = useAuth();
    const pubKey = uint8ArrayToZ32(auth?.publicKey!);

    async function selectChannel(channel: Channel) {
        await client.joinChannel(channel.id);
    }
</script>

<aside class="flex w-60 flex-col border-r">
    <div class="flex flex-col p-4 gap-2 justify-center items-center">
        <h1 class="text-center">{$currentServer?.title}</h1>
        <h3 class="text-center text-sm">{client.currentChannel?.name}</h3>
    </div>
    <hr />
    <div class="flex grow flex-col text-gray-400 gap-2 p-2">
        {#each channelList as channel}
            <button
                onclick={() => selectChannel(channel)}
                class={`cursor-pointer flex items-center p-2 rounded-md hover:bg-gray-800 ${client.currentChannel?.id === channel.id ? "bg-gray-800" : ""}`}
            >
                {#if channel.type === "Text"}
                    <span class="flex items-center gap-0.5"><Fa icon={faHashtag} /> {channel.name}</span>
                {:else}
                    <span class="flex items-center gap-0.5"><Fa icon={faVolume} /> {channel.name}</span>
                {/if}
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
