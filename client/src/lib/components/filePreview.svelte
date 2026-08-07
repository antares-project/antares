<script lang="ts">
    import {
        faFile,
        faMusic,
        faVideo,
    } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

    const { file }: { file: File } = $props();
</script>

<div
    class="flex items-center justify-center w-32 h-32 bg-gray-950/25 rounded-xl"
>
    {#if file.type.startsWith("image/")}
        <div class="flex items-center justify-center w-full h-full">
            <img
                class="w-full h-full"
                alt={file.name}
                src={URL.createObjectURL(file)}
            />
        </div>
    {:else if file.type.startsWith("audio/")}
        <div class="flex items-center justify-center w-full h-full">
            <Fa class="w-full h-full text-8xl" icon={faMusic} />
        </div>
    {:else if file.type.startsWith("video")}
        <div class="flex items-center justify-center w-full h-full">
            <Fa class="w-full h-full text-8xl" icon={faVideo} />
        </div>
    {:else}
        <div class="flex items-center justify-center w-full h-full">
            <Fa class="w-full h-full text-8xl" icon={faFile} />
        </div>
    {/if}
</div>
