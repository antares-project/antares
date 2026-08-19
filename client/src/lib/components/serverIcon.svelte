<script lang="ts">
	import { faTriangleExclamation } from "@fortawesome/free-solid-svg-icons";
	import { getInfo } from "harmon-lib/http";
	import { DNSClient } from "harmon-lib/pkdns";
	import Loading from "./loading.svelte";
	import Fa from "svelte-fa";

	const { onClick, publicKey } = $props();

	async function get_data() {
		const dnsClient = new DNSClient();

		const url = await dnsClient.resolveUrl(publicKey);

		if (!url) {
			throw new Error("Failed to resolve URL for public key: " + publicKey);
		}

		const data = await getInfo(url);

		return { data, url };
	}

	const data = get_data();
</script>

{#await data}
	<button
		onclick={onClick}
		class="group relative flex h-12 w-12 cursor-pointer flex-col items-center justify-center rounded-xl bg-zinc-600"
	>
		<Loading />
		<div
			class="pointer-events-none absolute left-full ml-2 flex translate-x-2 items-center justify-center rounded-md bg-slate-700 p-1 opacity-0 transition-all duration-200 group-hover:translate-x-0 group-hover:opacity-100"
		>
			<p>Loading...</p>
		</div>
	</button>
{:then { data, url }}
	<button
		onclick={onClick}
		class="group relative flex cursor-pointer flex-col items-center justify-center rounded-xl"
	>
		<img src={`${url}/icon`} alt="icon" class="h-12 w-12 rounded-xl" />
		<div
			class="pointer-events-none absolute left-full ml-2 flex translate-x-2 items-center justify-center rounded-md bg-slate-700 p-1 opacity-0 transition-all duration-200 group-hover:translate-x-0 group-hover:opacity-100"
		>
			<p>{data.title}</p>
		</div>
	</button>
{:catch error}
	<button
		onclick={onClick}
		class="group relative flex h-12 w-12 cursor-pointer flex-col items-center justify-center rounded-xl bg-zinc-600"
	>
		<Fa icon={faTriangleExclamation} />
		<div
			class="pointer-events-none absolute left-full ml-2 flex translate-x-2 items-center justify-center rounded-md bg-slate-700 p-1 opacity-0 transition-all duration-200 group-hover:translate-x-0 group-hover:opacity-100"
		>
			<p>{error.message}</p>
		</div>
	</button>
{/await}
