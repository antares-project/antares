<script lang="ts">
	import { faDownload, faFile } from "@fortawesome/free-solid-svg-icons";
	import type { Message } from "harmon-lib";
	import Fa from "svelte-fa";

	const { url, message }: { url: string; message: Message } = $props();

	// Encontra URLs no texto da mensagem:
	// https?:\/\/     -> exige o esquema "http://" ou "https://" (o ? se aplica ao caractere anterior)
	// [^\s<]+         -> corpo da URL: qualquer coisa menos espaco e "<"
	// [^\s<.,:;"')\]}]-> ultimo caractere; exclui pontuacao final para que "veja https://x.com." nao engula o ponto,
	// e "(https://x.com)" nao engula o parentese
	const URL_REGEX = /(https?:\/\/[^\s<]+[^\s<.,:;"')\]}])/gi;

	type ContentPart = { text: string; href?: string };

	function splitContent(content: string): ContentPart[] {
		const parts: ContentPart[] = [];
		let lastIndex = 0;
		for (const match of content.matchAll(URL_REGEX)) {
			const index = match.index ?? 0;
			if (index > lastIndex) {
				parts.push({ text: content.slice(lastIndex, index) });
			}
			parts.push({ text: match[0], href: match[0] });
			//Pula até o final do link, evitando também ter "sub link"
			lastIndex = index + match[0].length;
		}
		if (lastIndex < content.length) {
			parts.push({ text: content.slice(lastIndex) });
		}
		return parts;
	}

	const parts = $derived(splitContent(message.content));
</script>

<div class="flex flex-row gap-1 p-2 hover:bg-gray-800">
	<!-- Duas colunas, uma para a foto apenas -->
	<div
		class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full border border-blue-400 bg-blue-500 font-semibold text-white shadow-md shadow-black/40"
	>
		{message.profile.name[0]}
	</div>
	<div class="shrink">
		<p class="text-1xl text-gray-1 00 font-extrabold">{message.profile.name}</p>
		<p class="text-sm">
			{#each parts as part}
				{#if part.href}
					<a
						class="text-blue-400 hover:underline"
						href={part.href}
						target="_blank"
						rel="noopener noreferrer">{part.text}</a
					>
				{:else}
					{part.text}
				{/if}
			{/each}
		</p>
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
