<script lang="ts">
    import { fade } from "svelte/transition";
</script>

<div class="flex flex-row w-fit h-fit gap-2" transition:fade>
    <div class="size-4 bg-blue-800 rounded-full animate-bounce"></div>
    <div class="size-4 bg-blue-800 rounded-full animate-bounce" style="animation-delay: 100ms;"></div>
    <div class="size-4 bg-blue-800 rounded-full animate-bounce" style="animation-delay: 200ms;"></div>
</div>
