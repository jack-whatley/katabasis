<script lang="ts">
    import '../app.css'

    import Sidebar from '$lib/components/Sidebar.svelte';
    import Draggable from '$lib/components/Draggable.svelte';
    import NavBar from '$lib/components/NavBar.svelte';
    import Separator from '$lib/components/Separator.svelte';
    import NotificationSidebar from '$lib/components/NotificationSidebar.svelte';

    let { children } = $props();

    let notificationState = $state(false);
</script>

<Draggable />
<div class="w-screen h-screen flex flex-row">
    <Sidebar />
    <Separator horizontal={false}/>
    <div class="flex-1 flex flex-col">
        <NavBar bind:notificationState={notificationState}/>
        <Separator horizontal={true}/>
        <div class="flex-1 bg-zinc-800 text-white flex flex-row">
            {@render children()}
            <NotificationSidebar bind:isOpen={notificationState}/>
        </div>
    </div>
</div>
