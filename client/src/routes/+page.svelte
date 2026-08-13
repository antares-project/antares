<script lang="ts">
    import { goto } from "$app/navigation";
    import { useAuth } from "$lib/auth";
    import { info } from "$lib/log";
    import { sha256, sign } from "harmon-lib/crypto";
    import { stringToUint8Array } from "harmon-lib/utils";
    import { useStorage } from "$lib/storage.svelte";
    import { onMount } from "svelte";
    import { type ServerData } from "$lib/server.svelte";
    import { Client, type Channel } from "harmon-lib";
    import AddServerModal from "$lib/components/addServerModal.svelte";
    import Chat from "$lib/components/chat.svelte";
    import ChatsPanel from "$lib/components/chatsPanel.svelte";
    import SidePanel from "$lib/components/sidePanel.svelte";

    const auth = useAuth();

    const servers = useStorage<ServerData[]>("servers", []);
    const currentServer = useStorage<ServerData | undefined>("currentServer", undefined);

    let client: Client | null = $state(null);
    let channelList: Channel[] = $state([]);

    let showAddServerModal = $state(false);

    async function onClientConnect(client: Client) {
        info("onClientConnect");

        const challengeValue = await client.requestChallenge(auth?.publicKey!);

        const hash = sha256(stringToUint8Array(challengeValue.token));
        const signature = sign(hash, auth?.privateKey!);

        const confirmValue = await client.confirmChallenge(challengeValue.token, signature);

        await client.auth(confirmValue.token);

        channelList = await client.listChannels();
    }

    async function onClientDisconnect() {
        info("OnSocketDisconnect");
    }

    async function onCurrentServerChange(current: ServerData | undefined) {
        const url = current?.url;

        client?.close();

        if (!url) return;

        client = new Client(url);

        client.onOpen = async () => {
            onClientConnect(client!);
        };

        client.onClose = () => {
            onClientDisconnect();
        };
    }

    onMount(() => {
        if (!auth) {
            goto("/login");
        }
    });

    currentServer.subscribe(onCurrentServerChange);
</script>

<div class="w-screen h-screen bg-gray-900 text-white">
    {#if showAddServerModal}
        <AddServerModal
            onServerAdd={(v: ServerData) => {
                servers.update((s) => [...s, v]);
                showAddServerModal = false;
            }}
            onClose={() => {
                showAddServerModal = false;
            }}
        />
    {/if}
    {#if $servers.length == 0}
        <div class="flex flex-col items-center justify-center gap-4 w-full h-full">
            <h1 class="text-2xl">Nenhum servidor adicionado</h1>
            <button class="bg-gray-800 p-2 rounded-md text-white cursor-pointer" onclick={() => (showAddServerModal = true)}>
                Adicionar servidor
            </button>
        </div>
    {:else if $currentServer && client}
        <div class="grid w-full h-full grid-cols-[auto_auto_1fr_auto]">
            <SidePanel {servers} {currentServer} onAddServer={() => (showAddServerModal = true)} />
            <ChatsPanel {channelList} {currentServer} {client} />
            {#if client.currentChannel}
                {#key client.currentChannel}
                    <Chat {client} />
                {/key}
            {/if}
        </div>
    {:else}
        <div class="flex flex-row w-full h-full">
            <SidePanel {servers} {currentServer} onAddServer={() => (showAddServerModal = true)} />
            <div class="flex w-full h-full flex-col items-center justify-center gap-4">
                <h1 class="text-2xl text-center">Selecione um servidor</h1>
            </div>
        </div>
    {/if}
</div>
