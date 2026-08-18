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
			publicKey,
			url
		});
	}

	$effect(() => {
		updateInfo(publicKey ?? "");
	});
</script>

<div
	class="fixed top-0 left-0 z-10 flex h-screen w-screen flex-col items-center justify-center backdrop-blur-xs"
>
	<form
		class="relative flex flex-col items-center gap-2 rounded-md border border-gray-800 bg-gray-900 p-4"
		onsubmit={(event) => {
			event.preventDefault();
			submit();
		}}
	>
		<div class="absolute top-3 right-3 flex w-full flex-row justify-end">
			<button
				class="flex h-6 w-6 cursor-pointer items-center justify-center rounded-md p-1 hover:bg-white/10"
				type="button"
				onclick={onClose}
			>
				<Fa icon={faX}></Fa>
			</button>
		</div>
		{#if serverInfo}
			<div class="flex h-44 w-44 flex-col items-center justify-center">
				<img class="flex h-32 w-32 rounded-2xl" src={`${url}/icon`} alt="icon" />
				<h1 class="h-6 truncate text-white">{serverInfo?.title}</h1>
			</div>
		{:else}
			<Loading class="h-44 w-44" />
		{/if}
		<input
			bind:value={publicKey}
			class="rounded-md border-2 border-gray-700 bg-gray-800 p-2 text-center text-white"
			placeholder="PublicKey"
		/>
		<button
			type="submit"
			disabled={serverInfo == undefined}
			class="w-full cursor-pointer rounded-md bg-gray-800 p-2 text-white">Adicionar</button
		>
	</form>
</div>
