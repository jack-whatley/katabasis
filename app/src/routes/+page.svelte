<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    async function loadVersion(): Promise<string> {
        return await invoke<string>('get_version');
    }

    async function loadManagerVersion(): Promise<string> {
        return await invoke<string>('get_manager_version');
    }
</script>

<div class="w-full h-full bg-zinc-800 text-white p-2 flex flex-col select-none">
    <p>This is the home page for the katabasis mod manager, it is currently still being worked on. Go to 
        <a href="/collections" class="text-blue-700 hover:underline">this</a> 
        page to proceed.
    </p>
    <div class="mt-auto w-46 flex flex-col text-xs">
        <div class="flex flex-row">
            <p>katabasis::mm::app</p>
            <p class="ml-auto">v{#await loadVersion()}loading...{:then version}{version}{/await}</p>
        </div>
        <div class="flex flex-row">
            <p>katabasis::mm::manager</p>
            <p class="ml-auto">v{#await loadManagerVersion()}loading...{:then version}{version}{/await}</p>
        </div>
    </div>
</div>
