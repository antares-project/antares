<script lang="ts">
  import { faDownload, faFile } from "@fortawesome/free-solid-svg-icons";
  import type { Message } from "harmon-lib";
  import Fa from "svelte-fa";

  const { url, message }: { url: string; message: Message } = $props();
</script>

<div class="flex flex-row hover:bg-gray-800 p-2 gap-1">
  <!-- Duas colunas, uma para a foto apenas -->
  <div
    class="flex justify-center items-center shrink-0 w-8 h-8 rounded-full bg-blue-500"
  >
    {message.profile.name[0]}
  </div>
  <div class="shrink">
    <p class="text-1xl font-extrabold text-gray-1 00">{message.profile.name}</p>
    <p class="text-sm">{message.content}</p>
    <div class="flex flex-col items-start gap-4">
      {#each message.attachments as attachment}
        <div class="group flex relative max-h-96 mt-1">
          <a
            download={attachment.name}
            href={`${url}/files/${attachment.id}`}
            class="absolute hidden group-hover:flex z-10 -right-2 -top-2 bg-gray-900 p-1 rounded-sm cursor-pointer"
          >
            <Fa class="text-2xl" icon={faDownload} />
          </a>
          {#if attachment.mime_type.startsWith("audio")}
            <audio
              class="rounded-lg h-20"
              controls
              src={`${url}/files/${attachment.id}`}
            ></audio>
          {:else if attachment.mime_type.startsWith("image")}
            <img
              class="rounded-lg h-64"
              alt={attachment.hash}
              src={`${url}/files/${attachment.id}`}
            />
          {:else if attachment.mime_type.startsWith("video")}
            <video
              controls
              class="rounded-lg h-64"
              src={`${url}/files/${attachment.id}`}
            >
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
</div>
