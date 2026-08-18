<script lang="ts">
	import { faFile, faMusic, faVideo } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";

	const { file }: { file: File } = $props();
</script>

<div class="flex h-32 w-32 items-center justify-center rounded-xl bg-gray-950/25">
	{#if file.type.startsWith("image/")}
		<div class="flex h-full w-full items-center justify-center">
			<img class="h-full w-full" alt={file.name} src={URL.createObjectURL(file)} />
		</div>
	{:else if file.type.startsWith("audio/")}
		<div class="flex h-full w-full items-center justify-center">
			<Fa class="h-full w-full text-8xl" icon={faMusic} />
		</div>
	{:else if file.type.startsWith("video")}
		<div class="flex h-full w-full items-center justify-center">
			<Fa class="h-full w-full text-8xl" icon={faVideo} />
		</div>
	{:else}
		<div class="flex h-full w-full items-center justify-center">
			<Fa class="h-full w-full text-8xl" icon={faFile} />
		</div>
	{/if}
</div>
