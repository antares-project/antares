<script lang="ts">
	import "./layout.css";
	import icon from "$lib/assets/icon.png";
	import { type Snippet } from "svelte";
	import Toast from "$lib/components/toast.svelte";

	let { children }: { children: Snippet } = $props();
</script>

<svelte:head>
	<title>Harmon</title>
	<link rel="icon" href={icon} />
</svelte:head>

<Toast />

{@render children?.()}
