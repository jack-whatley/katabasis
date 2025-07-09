<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    async function isFirstTime(): Promise<boolean> {
        return await invoke("is_first_time");
    }

    let welcomeText = $state("Welcome!");

    onMount(async () => await isFirstTime() ? welcomeText = welcomeText : welcomeText = "Welcome back!");
</script>

<div class="w-full h-full flex flex-col select-none p-2">
    <p class="text-2xl">{welcomeText}</p>
</div>
