<script lang="ts">
    import { type ServerData } from "$lib/server.svelte";

    const {
        data,
        onClick,
    }: {
        data: ServerData;
        onClick: () => void;
    } = $props();
</script>

<button
    onclick={onClick}
    class="relative flex flex-col items-center justify-center group cursor-pointer"
>
    <img src={`${data.url}/icon`} alt="icon" class="h-12 w-12" />
    <div
        class="absolute bg-slate-700 p-1 rounded-md items-center justify-center flex left-full ml-2 opacity-0 translate-x-2 transition-all duration-200 group-hover:opacity-100 group-hover:translate-x-0 pointer-events-none"
    >
        <p>{data.title}</p>
    </div>
</button>
