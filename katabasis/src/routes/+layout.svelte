<script lang="ts">
    import "../app.css";
    import WindowChrome from "$lib/menu/WindowChrome.svelte";
    import Sidebar from "$lib/menu/Sidebar.svelte";
    import WindowDrag from "$lib/menu/WindowDrag.svelte";

    let { children } = $props();

    document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<div class="w-screen h-screen bg-neutral-800 flex flex-row inter-400 text-white">
    <WindowDrag />
    <Sidebar />
    <div class="flex flex-col flex-1">
        <WindowChrome />
        {@render children()}
    </div>
</div>
