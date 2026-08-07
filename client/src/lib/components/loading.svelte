<script lang="ts">
	import { faCog } from "@fortawesome/free-solid-svg-icons";
	import Fa from "svelte-fa";

	const { class: className }: { class?: string } = $props();
</script>

<div class={"flex justify-center items-center " + (className ?? "")}>
	<Fa spin icon={faCog} size="3x" />
</div>
