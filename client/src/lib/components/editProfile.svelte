<script lang="ts">
	import { faClose } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";

	const {
		onEdit,
		onClose,
		closable
	}: {
		onEdit?: (name: string) => void;
		onClose?: () => void;
		closable?: boolean;
	} = $props();

	let name = $state("");

	async function submit(event: SubmitEvent) {
		event.preventDefault();
		onEdit?.(name);
	}
</script>

<div class="fixed z-10 flex h-screen w-screen items-center justify-center backdrop-blur-xs">
	<form
		class="relative flex flex-col items-center justify-center gap-4 rounded-md border border-gray-800 bg-gray-900 p-4"
		onsubmit={submit}
	>
		{#if closable ?? false}
			<button
				type="button"
				class="absolute top-4 right-4 cursor-pointer text-white"
				onclick={onClose}
			>
				<Fa icon={faClose}></Fa>
			</button>
		{/if}
		<h1 class="text-lg text-white">Edit Profile</h1>
		<div class="flex flex-col gap-2">
			<label for="username" class="text-white">Username</label>
			<input
				bind:value={name}
				id="username"
				type="text"
				class="rounded-md bg-gray-800 p-2 text-white"
			/>
		</div>
		<button type="submit" class="w-full cursor-pointer rounded-md bg-gray-800 p-2 text-white"
			>Save</button
		>
	</form>
</div>
