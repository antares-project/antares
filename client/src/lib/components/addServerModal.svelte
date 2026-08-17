<script lang="ts">
    import { getInfo, type GetInfoResponse } from "harmon-lib/http";
    import { faX } from "@fortawesome/free-solid-svg-icons";
    import Loading from "./loading.svelte";
    import Fa from "svelte-fa";
    import { DNSClient } from "harmon-lib/pkdns";

    let url = $state("");
    let publicKey = $state("");
    let serverInfo: GetInfoResponse | undefined = $state();

    const { onServerAdd, onClose } = $props();

    async function updateInfo(publicKey: string) {
        try {
            const dnsClient = new DNSClient();
            const urlEntry = await dnsClient.resolveUrl(`https://${publicKey}`);

            if (!urlEntry) return;

            url = urlEntry;
            serverInfo = await getInfo(url);
        } catch {
            serverInfo = undefined;
        }
    }

    function submit() {
        onServerAdd({
            ...serverInfo!,
            url,
        });
    }

    $effect(() => {
        updateInfo(publicKey ?? "");
    });
</script>

<div class="fixed top-0 left-0 backdrop-blur-xs z-10 flex flex-col w-screen h-screen items-center justify-center">
    <form
        class="relative flex flex-col items-center bg-gray-900 gap-2 p-4 border border-gray-800 rounded-md"
        onsubmit={(event) => {
            event.preventDefault();
            submit();
        }}
    >
        <div class="absolute top-3 right-3 flex w-full flex-row justify-end">
            <button class="flex hover:bg-white/10 p-1 rounded-md w-6 h-6 items-center justify-center cursor-pointer" type="button" onclick={onClose}>
                <Fa icon={faX}></Fa>
            </button>
        </div>
        {#if serverInfo}
            <div class="flex flex-col w-44 h-44 items-center justify-center">
                <img class="flex rounded-2xl w-32 h-32" src={`${url}/icon`} alt="icon" />
                <h1 class="text-white h-6 truncate">{serverInfo?.title}</h1>
            </div>
        {:else}
            <Loading class="w-44 h-44" />
        {/if}
        <input bind:value={publicKey} class="text-white p-2 rounded-md border-2 border-gray-700 bg-gray-800 text-center" placeholder="PublicKey" />
        <button type="submit" disabled={serverInfo == undefined} class="w-full bg-gray-800 text-white p-2 rounded-md cursor-pointer">Adicionar</button
        >
    </form>
</div>
