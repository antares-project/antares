<script lang="ts">
	import { faDownload, faFile } from "@fortawesome/free-solid-svg-icons";
	import type { Message } from "harmon-lib";
	import Fa from "svelte-fa";

	const { url, message }: { url: string; message: Message } = $props();
</script>

<div class="flex flex-row gap-1 p-2 hover:bg-gray-800">
	<!-- Duas colunas, uma para a foto apenas -->
	<div class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-blue-500">
		{message.profile.name[0]}
	</div>
	<div class="shrink">
		<p class="text-1xl text-gray-1 00 font-extrabold">{message.profile.name}</p>
		<p class="text-sm">{message.content}</p>
		<div class="flex flex-col items-start gap-4">
			{#each message.attachments as attachment}
				<div class="group relative mt-1 flex max-h-96">
					<a
						download={attachment.name}
						href={`${url}/files/${attachment.id}`}
						class="absolute -top-2 -right-2 z-10 hidden cursor-pointer rounded-sm bg-gray-900 p-1 group-hover:flex"
					>
						<Fa class="text-2xl" icon={faDownload} />
					</a>
					{#if attachment.mime_type.startsWith("audio")}
						<audio
							class="h-20 rounded-lg"
							controls
							src={`${url}/files/${attachment.id}`}
						></audio>
					{:else if attachment.mime_type.startsWith("image")}
						<img
							class="h-64 rounded-lg"
							alt={attachment.hash}
							src={`${url}/files/${attachment.id}`}
						/>
					{:else if attachment.mime_type.startsWith("video")}
						<video
							controls
							class="h-64 rounded-lg"
							src={`${url}/files/${attachment.id}`}
						>
							<track kind="captions" />
						</video>
					{:else}
						<div class="flex gap-2 rounded-lg p-2 hover:bg-gray-900">
							<Fa class="text-5xl" icon={faFile} />
							<p>{attachment.name}</p>
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</div>
</div>
