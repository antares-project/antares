<script lang="ts">
    import { getInfo, type GetInfoResponse } from "antares-lib/http";
    import { DNSClient, ResolvePolicy, type HTTPSTypeRecordData } from "antares-lib/pkdns";
    import { faX } from "@fortawesome/free-solid-svg-icons";
    import Loading from "./loading.svelte";
    import Fa from "svelte-fa";

    let url = $state("");
    let publicKey = $state("");
    let serverInfo: GetInfoResponse | undefined = $state();

    const { onServerAdd, onClose } = $props();

    async function updateInfo(publicKey: string) {
        try {
            const dnsClient = new DNSClient();
            const entries = await dnsClient.resolve(publicKey, ResolvePolicy.NetworkOnly);
            const entry = entries?.find((entry) => entry.rdata.type == "HTTPS");
            const record = entry?.rdata as HTTPSTypeRecordData | undefined;

            if (!record) return;

            url = `https://${record.target}`;
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
        class="relative flex flex-col items-center bg-gray-900 gap-2 p-4 border border-gray-800"
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
            <div class="flex flex-col w-40 h-40 items-center justify-center">
                <img class="flex h-full" src={`${url}/icon`} alt="icon" />
                <h1 class="text-white h-6">{serverInfo?.title}</h1>
            </div>
        {:else}
            <Loading class="w-40 h-40" />
        {/if}
        <input bind:value={publicKey} class="text-white p-2 rounded-md border-2 border-gray-700 bg-gray-800" placeholder="PublicKey" />
        <button type="submit" disabled={serverInfo == undefined} class="w-full bg-gray-800 text-white p-2 rounded-md cursor-pointer">Adicionar</button
        >
    </form>
</div>
