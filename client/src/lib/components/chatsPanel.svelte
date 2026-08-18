<script lang="ts">
	import type { ServerData } from "$lib/server.svelte";
	import type { Channel, Client, Profile } from "harmon-lib";
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
		onClickProfile
	}: {
		client: Client;
		channelList: Channel[];
		currentServer: Writable<ServerData | undefined>;
		onClickProfile: () => void;
	} = $props();

	const auth = useAuth();
	const pubKey = uint8ArrayToZ32(auth?.publicKey!);

	async function selectChannel(channel: Channel) {
		await client.joinChannel(channel.id);
	}
</script>

<aside class="flex w-60 flex-col border-r">
	<div class="flex flex-col items-center justify-center gap-2 p-4">
		<h1 class="text-center">{$currentServer?.title}</h1>
		<h3 class="text-center text-sm">{client.currentChannel?.name}</h3>
	</div>
	<hr />
	<div class="flex grow flex-col gap-2 p-2 text-gray-400">
		{#each channelList as channel}
			<button
				onclick={() => selectChannel(channel)}
				class={`flex cursor-pointer items-center rounded-md p-2 hover:bg-gray-800 ${client.currentChannel?.id === channel.id ? "bg-gray-800" : ""}`}
			>
				{#if channel.type === "Text"}
					<span class="flex items-center gap-0.5"
						><Fa icon={faHashtag} /> {channel.name}</span
					>
				{:else}
					<span class="flex items-center gap-0.5"
						><Fa icon={faVolume} /> {channel.name}</span
					>
				{/if}
			</button>
		{/each}
	</div>
	<hr />
	<div class="flex h-20 flex-col justify-center p-4">
		<button class="cursor-pointer truncate" onclick={onClickProfile}>
			<p class="text-lg">{client.profile?.name}</p>
		</button>
		<button
			class="cursor-pointer truncate"
			onclick={() => {
				navigator.clipboard.writeText(pubKey);
				push("PublicKey copied to clipboard");
			}}
		>
			<p class="text-xs">{pubKey}</p>
		</button>
	</div>
</aside>
