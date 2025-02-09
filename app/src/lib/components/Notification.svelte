<script lang="ts">
    import { NotificationModel, getNotificationColour } from "$lib/utils/notification";

    let { notification, clickCallback }: { notification: NotificationModel, clickCallback: () => void } = $props();
    let notificationColour = getNotificationColour(notification.level);
</script>

<button class="cursor-pointer flex flex-col items-start justify-stretch bg-zinc-800 w-full p-2 rounded border-l-4" 
        onclick={clickCallback}
        title="Click to dismiss notification..."
        style="border-color: {notificationColour}">
    <p class="text-sm select-none">{notification.title}</p>
    <p class="text-xs select-none text-left">{notification.body}</p>
</button>
