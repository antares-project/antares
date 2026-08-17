<script lang="ts">
    import { faClose } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

    const {
        onEdit,
        onClose,
        closable,
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

<div class="flex items-center justify-center w-screen h-screen fixed z-10 backdrop-blur-xs">
    <form class="relative flex flex-col items-center justify-center gap-4 bg-gray-900 p-4 border border-gray-800 rounded-md" onsubmit={submit}>
        {#if closable ?? false}
            <button type="button" class="absolute top-4 right-4 text-white" onclick={onClose}>
                <Fa icon={faClose}></Fa>
            </button>
        {/if}
        <h1 class="text-white text-lg">Edit Profile</h1>
        <div class="flex flex-col gap-2">
            <label for="username" class="text-white">Username</label>
            <input bind:value={name} id="username" type="text" class="bg-gray-800 text-white p-2 rounded-md" />
        </div>
        <button type="submit" class="w-full bg-gray-800 text-white p-2 rounded-md cursor-pointer">Save</button>
    </form>
</div>
