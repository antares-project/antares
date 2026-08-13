<script lang="ts">
    import { clamp } from "$lib/utils";
    import { englishWordlist as wordlist } from "harmon-lib/crypto";

    const {
        onWordAdd,
        onWordRemove,
        listSize = 12,
        words,
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

<div class="w-full max-w-md bg-gray-800 text-white rounded-xl">
    <div class="flex flex-wrap items-center gap-2 p-3 rounded-xl border shadow-sm">
        {#each words as word}
            <span class="bg-gray-700 text-blue-100 px-2 py-1 rounded-lg text-sm">
                {word}
            </span>
        {/each}

        <input
            bind:value={input}
            onkeydown={handleKeydown}
            class="flex-1 min-w-30 outline-none p-1"
            placeholder={words.length === 0 ? `Type the ${listSize} words...` : ""}
        />
    </div>

    {#if suggestions.length && input && words.length < listSize}
        <div class="mt-2 bg-zinc-900 border rounded-xl shadow-sm max-h-40 overflow-auto">
            {#each suggestions as suggestion, i}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div class="px-3 py-2 cursor-pointer {i === selected ? 'bg-zinc-700' : 'hover:bg-zinc-800'}" onclick={() => addSelected(i)}>
                    {suggestion}
                </div>
            {/each}
        </div>
    {/if}
</div>
