<script lang="ts">
	import { onMount, tick } from "svelte";
	import { Client, type Message } from "harmon-lib";
	import Fa from "svelte-fa";
	import { faUpload } from "@fortawesome/free-solid-svg-icons";
	import { postFiles } from "harmon-lib/http";
	import FilePreview from "./filePreview.svelte";
	import Msg from "./message.svelte";
	import Loading from "./loading.svelte";

	let { client }: { client: Client } = $props();

	let messages = $state<Message[]>([]);
	let messageContent = $state("");
	let messageFiles: FileList | undefined = $state();

	let scrollContainer: HTMLDivElement | null = null;
	let isLoadingMessages = false;
	let isSendingMessage = $state(false);
	let hasMore = true;

	const beforeId = $derived<string | undefined>(messages?.[0]?.id);

	async function scrollToBottom() {
		await tick();
		if (scrollContainer) {
			scrollContainer.scrollTop = scrollContainer.scrollHeight;
		}
	}

	async function sendMessage() {
		if ((!messageContent.trim() && !messageFiles) || isSendingMessage) return;

		try {
			isSendingMessage = true;

			if (messageFiles) {
				const files = await postFiles(client.url, messageFiles);
				const fileIds = files.map((f) => f.id);

				await client.sendMessage(messageContent, fileIds);
			} else {
				await client.sendMessage(messageContent, []);
			}
		} finally {
			messageContent = "";
			messageFiles = undefined;
			isSendingMessage = false;
		}
	}

	async function messageReceived(message: Message) {
		messages = [...messages, message];
		await scrollToBottom();
	}

	async function messagesLoaded(msgs: Message[]) {
		hasMore = msgs.length > 0;

		if (scrollContainer) {
			const previousScrollHeight = scrollContainer.scrollHeight;
			const previousScrollTop = scrollContainer.scrollTop;

			messages = [...msgs, ...messages];

			await tick();

			scrollContainer.scrollTop =
				scrollContainer.scrollHeight - previousScrollHeight + previousScrollTop;
		} else {
			messages = [...msgs, ...messages];
		}
	}

	async function loadMessages() {
		if (isLoadingMessages || !hasMore) return;

		isLoadingMessages = true;

		const messages = await client.loadMessages(beforeId);

		await messagesLoaded(messages);

		isLoadingMessages = false;
	}

	function onScroll() {
		if (!scrollContainer) return;

		if (scrollContainer.scrollTop <= 24) {
			loadMessages();
		}
	}

	onMount(() => {
		client.onMessageReceived = messageReceived;

		loadMessages();
	});
</script>

<div class="flex h-screen w-full flex-col">
	<div class="flex h-full w-full overflow-y-auto">
		<div
			bind:this={scrollContainer}
			onscroll={onScroll}
			class="flex h-full w-full overflow-y-auto font-mono"
		>
			<div class="flex w-full flex-col">
				{#each messages as msg}
					<Msg message={msg} url={client.url} />
				{/each}
			</div>
		</div>
	</div>
	{#if isSendingMessage}
		<hr />
		<div class="flex h-20 w-full shrink-0 items-center justify-center">
			<Loading />
		</div>
	{:else}
		{#if messageFiles}
			<hr />
			<div class="flex flex-row gap-2 p-2">
				{#each messageFiles as file}
					<FilePreview {file} />
				{/each}
			</div>
			<hr />
		{:else}
			<hr />
		{/if}
		<form
			class="flex h-20 w-full shrink-0 flex-col items-center justify-center gap-2 p-2"
			onsubmit={(e) => {
				e.preventDefault();
				sendMessage();
			}}
		>
			<div class="flex w-full flex-row items-center">
				<label
					class="flex h-10 w-10 cursor-pointer items-center justify-center"
					for="file-upload"
				>
					<Fa class="text-2xl" icon={faUpload}></Fa>
					<input
						bind:files={messageFiles}
						class="hidden"
						type="file"
						id="file-upload"
						multiple
					/>
				</label>
				<textarea
					bind:value={messageContent}
					onkeydown={(event) => {
						if (event.key === "Enter" && !event.shiftKey) {
							event.preventDefault();
							sendMessage();
						}
					}}
					class="w-full rounded-md border border-inherit bg-inherit p-1.5"></textarea>
				<button type="submit" class="cursor-pointer text-3xl">▶️</button>
			</div>
		</form>
	{/if}
</div>
