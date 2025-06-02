<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { CaretRight } from "phosphor-svelte";
</script>

<div class="w-full h-full bg-zinc-800 text-white p-2 flex flex-col select-none">
    <p class="text-2xl mb-2">Welcome back!</p>
    <a class="text-xl mb-2 flex flex-row items-center hover:underline" href="/collections">
        Recently played
        <CaretRight class="ml-1" />
    </a>
</div>
