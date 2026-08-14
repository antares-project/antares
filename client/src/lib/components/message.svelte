<script lang="ts">
    import { faDownload, faFile } from "@fortawesome/free-solid-svg-icons";
    import type { Message } from "harmon-lib";
    import Fa from "svelte-fa";

    const { url, message }: { url: string; message: Message } = $props();
</script>

<div class="flex flex-col hover:bg-gray-800 p-4">
    <p>{message.content}</p>
    <div class="flex flex-col items-start gap-4">
        {#each message.attachments as attachment}
            <div class="group flex relative max-h-96">
                <a
                    download={attachment.name}
                    href={`${url}/files/${attachment.id}`}
                    class="absolute hidden group-hover:flex z-10 -right-2 -top-2 bg-gray-900 p-1 rounded-sm cursor-pointer"
                >
                    <Fa class="text-2xl" icon={faDownload} />
                </a>
                {#if attachment.mime_type.startsWith("audio")}
                    <audio class="rounded-lg" controls src={`${url}/files/${attachment.id}`}></audio>
                {:else if attachment.mime_type.startsWith("image")}
                    <img class="rounded-lg" alt={attachment.hash} src={`${url}/files/${attachment.id}`} />
                {:else if attachment.mime_type.startsWith("video")}
                    <video controls class="rounded-lg" src={`${url}/files/${attachment.id}`}>
                        <track kind="captions" />
                    </video>
                {:else}
                    <div class="flex gap-2 p-2 hover:bg-gray-900 rounded-lg">
                        <Fa class="text-5xl" icon={faFile} />
                        <p>{attachment.name}</p>
                    </div>
                {/if}
            </div>
        {/each}
    </div>
</div>
