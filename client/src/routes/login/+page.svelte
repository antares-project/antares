<script lang="ts">
	import { goto } from "$app/navigation";
	import { login } from "$lib/auth";
	import InputWords from "$lib/components/inputWords.svelte";
	import { keygen, mnemonicToSeed, generateMnemonic } from "harmon-lib/crypto";

	const listSize = 12;

	let words = $state<string[]>([]);

	function onWordAdd(word: string) {
		if (words.length <= listSize) {
			words = [...words, word];
		}
	}

	function onWordRemove(word: string) {
		words = words.filter((w) => w !== word);
	}

	async function onSubmit() {
		const seed = await mnemonicToSeed(words);
		const { privateKey, publicKey } = keygen(seed);
		login(publicKey, privateKey);
		goto("/");
	}
</script>

<div class="flex h-screen w-screen items-center justify-center bg-gray-900 text-white">
	<div>
		<h1 class="mb-4 text-center text-2xl font-bold">Login</h1>
		<div class="flex flex-col gap-4">
			<InputWords {onWordAdd} {onWordRemove} {listSize} {words} />
			<input
				type="button"
				value="Generate"
				onclick={() => {
					words = generateMnemonic();
				}}
				class="cursor-pointer rounded bg-green-600 px-4 py-2 font-bold text-white hover:bg-green-700"
			/>
			<input
				onclick={onSubmit}
				type="button"
				value="Login"
				disabled={words.length < listSize}
				class="cursor-pointer rounded bg-blue-600 px-4 py-2 font-bold text-white hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-50"
			/>
		</div>
	</div>
</div>
