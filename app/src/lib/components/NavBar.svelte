<script lang="ts">
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { Icon, Icons } from "$lib/icons";
    import NavButton from "./NavButton.svelte";

    let { notificationState = $bindable(false) } = $props();

    function minimiseApp() {
        getCurrentWindow().minimize();
    }

    function closeApp() {
        getCurrentWindow().close();
    }
</script>

<div class="bg-zinc-800 w-full h-[45px] text-white flex flex-col">
    <div class="ml-auto z-50 h-full gap-0 flex flex-row">
        <NavButton isClose={false} clickCallback={minimiseApp}>
            <Icon colour="#FFFFFF" icon={Icons.Minimise}/>
        </NavButton>
        <NavButton isClose={false} clickCallback={() => notificationState = !notificationState}>
            <Icon colour="#FFFFFF" icon={Icons.Notification}/>
        </NavButton>
        <NavButton isClose={true} clickCallback={closeApp}>
            <Icon colour="#FFFFFF" icon={Icons.Close}/>
        </NavButton>
    </div>
</div>
