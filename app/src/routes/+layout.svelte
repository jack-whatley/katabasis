<script lang="ts">
    import '../app.css'

    import Sidebar from '$lib/components/Sidebar.svelte';
    import Draggable from '$lib/components/Draggable.svelte';

    let { children } = $props();
</script>

<Draggable />
<div class="w-screen h-screen flex flex-row">
    <Sidebar/>
    <div class="flex-1">
        {@render children()}
    </div>
</div>
