<script lang="ts">
    import { NotificationLevel, NotificationModel } from "$lib/utils/notification";
    import Notification from "./Notification.svelte";

    let { isOpen = $bindable(false) } = $props();
    let width = $derived(isOpen ? "w-1/3" : "w-[0px]");

    async function delayOpen() {
        await new Promise(resolve => setTimeout(resolve, 30));
    }

    function removeItem(array: Array<NotificationModel>, item: NotificationModel) {
        var index = array.indexOf(item);

        if (index !== -1) {
            array.splice(index, 1);
        }
    }

    let notifications: Array<NotificationModel> = $state([
        new NotificationModel("Notification One", 
        "here is the notification body, this may be really large due to an exteme error message", 
        NotificationLevel.Info)
    ]);
</script>

<div class="flex flex-col h-full bg-neutral-900 ml-auto transition-[width] duration-75 ease-in-out {width}">
    {#if isOpen}
    {#await delayOpen() then value}
    <p class="w-full border-b border-zinc-800 p-2 select-none">Notifications</p>
    <div class="flex flex-1 flex-col p-2 gap-2">
        {#each notifications as note}
        <Notification notification={note} clickCallback={() => removeItem(notifications, note)}/>
        {:else}
        <p class="m-auto text-xs select-none">Notifications empty 😴</p>
        {/each}
    </div>
    {/await}
    {/if}
</div>
