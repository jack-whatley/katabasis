<script lang="ts">
    import '../app.css'
    import { Sidebar, NavBar, Draggable, Separator, NotificationSidebar } from '$lib/components';

    let { children } = $props();

    let notificationMenuOpen = $state(false);
</script>

<Draggable />
<div class="w-screen h-screen flex flex-row">
    <Sidebar />
    <Separator horizontal={false}/>
    <div class="flex-1 flex flex-col">
        <NavBar bind:notificationState={notificationMenuOpen}/>
        <Separator horizontal={true}/>
        <div class="flex-1 bg-zinc-800 text-white flex flex-row">
            {@render children()}
            <NotificationSidebar bind:isOpen={notificationMenuOpen}/>
        </div>
    </div>
</div>
