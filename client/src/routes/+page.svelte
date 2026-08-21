<script lang="ts">
	import { goto } from "$app/navigation";
	import { useAuth } from "$lib/auth";
	import { info } from "$lib/log";
	import { sha256, sign } from "harmon-lib/crypto";
	import { stringToUint8Array, z32toUint8Array } from "harmon-lib/utils";
	import { useStorage } from "$lib/storage.svelte";
	import { onMount } from "svelte";
	import { Client } from "harmon-lib";
	import AddServerModal from "$lib/components/addServerModal.svelte";
	import Chat from "$lib/components/chat.svelte";
	import ChatsPanel from "$lib/components/chatsPanel.svelte";
	import SidePanel from "$lib/components/sidePanel.svelte";
	import EditProfile from "$lib/components/editProfile.svelte";

	const auth = useAuth();

	const servers = useStorage<string[]>("servers", []);
	const currentServer = useStorage<string | undefined>("currentServer", undefined);

	let client: Client | undefined = $state();

	let showAddServerModal = $state(false);
	let isEditingProfile = $state(false);

	async function onClientConnect(client: Client) {
		info("client connect as id: " + client.id);

		const challengeValue = await client.requestChallenge(auth?.publicKey!);

		const hash = sha256(stringToUint8Array(challengeValue.token));
		const signature = sign(hash, auth?.privateKey!);

		const confirmValue = await client.confirmChallenge(challengeValue.token, signature);

		await client.auth(confirmValue.token);

		isEditingProfile = client.profile == undefined;
	}

	async function onClientDisconnect() {
		info("OnSocketDisconnect");
	}

	async function onCurrentServerChange(z32publicKey: string | undefined) {
		client?.close();

		if (!z32publicKey) return;

		const publicKey = z32toUint8Array(z32publicKey);

		client = await Client.init(publicKey);

		client.onConnectionReady = async () => {
			await onClientConnect(client!);
		};

		client.onConnectionClosed = async () => {
			await onClientDisconnect();
		};
	}

	onMount(() => {
		if (!auth) {
			goto("/login");
			return;
		}

		currentServer.subscribe(onCurrentServerChange);
	});
</script>

<div class="h-screen w-screen bg-gray-900 text-white">
	{#if isEditingProfile}
		<EditProfile
			onEdit={async (name) => {
				await client?.updateProfile(name);
				isEditingProfile = false;
			}}
			closable={!!client?.profile}
			onClose={() => {
				isEditingProfile = false;
			}}
		/>
	{/if}
	{#if showAddServerModal}
		<AddServerModal
			onServerAdd={(pubKey: string) => {
				servers.update((p) => [...p, pubKey]);
				showAddServerModal = false;
			}}
			onClose={() => {
				showAddServerModal = false;
			}}
		/>
	{/if}
	{#if $servers.length == 0}
		<div class="flex h-full w-full flex-col items-center justify-center gap-4">
			<h1 class="text-2xl">Nenhum servidor adicionado</h1>
			<button
				class="cursor-pointer rounded-md bg-gray-800 p-2 text-white"
				onclick={() => {
					showAddServerModal = true;
				}}
			>
				Adicionar servidor
			</button>
		</div>
	{:else if client}
		<div class="grid h-full w-full grid-cols-[auto_auto_1fr_auto]">
			<SidePanel
				{servers}
				{currentServer}
				onAddServer={() => {
					showAddServerModal = true;
				}}
			/>
			<ChatsPanel
				{client}
				onClickProfile={() => {
					isEditingProfile = true;
				}}
			/>
			{#if client.currentChannel}
				{#key client.currentChannel}
					<Chat {client} />
				{/key}
			{/if}
		</div>
	{:else}
		<div class="flex h-full w-full flex-row">
			<SidePanel
				{servers}
				{currentServer}
				onAddServer={() => {
					showAddServerModal = true;
				}}
			/>
			<div class="flex h-full w-full flex-col items-center justify-center gap-4">
				<h1 class="text-center text-2xl">Selecione um servidor</h1>
			</div>
		</div>
	{/if}
</div>
