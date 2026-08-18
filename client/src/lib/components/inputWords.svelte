<script lang="ts">
	import { clamp } from "$lib/utils";
	import { englishWordlist as wordlist } from "harmon-lib/crypto";

	const {
		onWordAdd,
		onWordRemove,
		listSize = 12,
		words
	}: {
		onWordAdd?: (word: string) => void;
		onWordRemove?: (word: string) => void;
		listSize?: number;
		words: string[];
	} = $props();

	let input = $state("");
	let selected = $state(0);

	const suggestions = $derived(wordlist.filter((s) => s.startsWith(input) && !words.includes(s)));

	$effect(() => {
		selected = clamp(selected, 0, suggestions.length - 1);
	});

	function addSelected(index = selected) {
		const word = suggestions[index];
		if (!word || !input.trim()) return;

		input = "";
		selected = 0;

		onWordAdd?.(word);
	}

	function handleKeydown(event: KeyboardEvent) {
		if (event.key === "Enter" && suggestions.length > 0 && words.length < listSize) {
			addSelected();
			return;
		}

		if (event.key === "ArrowDown") {
			event.preventDefault();
			selected = (selected + 1) % suggestions.length;
			return;
		}

		if (event.key === "ArrowUp") {
			event.preventDefault();
			selected = (selected - 1) % suggestions.length;
			return;
		}

		if (event.key === "Backspace" && input === "") {
			onWordRemove?.(words[words.length - 1]);
			return;
		}

		if (words.length >= listSize) {
			event.preventDefault();
		}
	}
</script>

<div class="w-full max-w-md rounded-xl bg-gray-800 text-white">
	<div class="flex flex-wrap items-center gap-2 rounded-xl border p-3 shadow-sm">
		{#each words as word}
			<span class="rounded-lg bg-gray-700 px-2 py-1 text-sm text-blue-100">
				{word}
			</span>
		{/each}

		<input
			bind:value={input}
			onkeydown={handleKeydown}
			class="min-w-30 flex-1 p-1 outline-none"
			placeholder={words.length === 0 ? `Type the ${listSize} words...` : ""}
		/>
	</div>

	{#if suggestions.length && input && words.length < listSize}
		<div class="mt-2 max-h-40 overflow-auto rounded-xl border bg-zinc-900 shadow-sm">
			{#each suggestions as suggestion, i}
				<!-- svelte-ignore a11y_click_events_have_key_events -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class="cursor-pointer px-3 py-2 {i === selected ? 'bg-zinc-700' : 'hover:bg-zinc-800'}"
					onclick={() => addSelected(i)}
				>
					{suggestion}
				</div>
			{/each}
		</div>
	{/if}
</div>
