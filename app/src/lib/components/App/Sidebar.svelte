<script lang="ts">
    import AppTitle from "./AppTitle.svelte";
    import SidebarButton from "./SidebarButton.svelte";

    import { sidebarItems, settingsItem } from "$lib/utils/index";
    import { Icon } from "$lib/icons";

    import StackPlus from "phosphor-svelte/lib/StackPlus";

    import Dialogue from "../Dialogue.svelte";
    import TextInput from "../TextInput.svelte";

    import * as Select from "$lib/components/ui/select/index";
    import * as Tabs from "$lib/components/ui/tabs";
    import { Button } from "$lib/components/ui/button/index"
    import {invoke} from "@tauri-apps/api/core";

    let games = [
        { value: "lethal-company", label: "Lethal Company" }
    ];

    let collectionName = $state("");
    let selectedGame = $state("");

    const dropdownLabel = $derived(
        games.find((f) => f.value === selectedGame)?.label ?? "Select a game..."
    );

    async function createCollection() {
        await invoke("create_collection", { name: collectionName, game: selectedGame });
    }
</script>

{#snippet newTab()}
    <div class="select-none flex flex-col p-1">
        <p class="text-lg">New Collection</p>
        <p class="text-sm mt-2 mb-1">Name</p>
        <TextInput bind:value={collectionName} class="w-full" />
        <p class="text-sm mt-2 mb-1">Game</p>
        <Select.Root type="single" name="collectionGame" bind:value={selectedGame}>
            <Select.Trigger class="w-full">
                {dropdownLabel}
            </Select.Trigger>
            <Select.Content>
                {#each games as game}
                    <Select.Item value={game.value}>{game.label}</Select.Item>
                {/each}
            </Select.Content>
        </Select.Root>
        <Button variant="default" class="ml-auto w-26 mt-2"
            onclick={async () => await createCollection()}>
            Create
        </Button>
    </div>
{/snippet}

{#snippet importTab()}
    <div>import tab</div>
{/snippet}

<div class="text-white min-w-1/6 bg-neutral-900 flex flex-col">
    <AppTitle/>
    {#each sidebarItems as item}
        <SidebarButton url={item.url}>
            <Icon colour="#FFFFFF" icon={item.icon}/>
            <p>{item.display}</p>
        </SidebarButton>
    {/each}
    <div class="mt-auto w-full flex">
        <Dialogue title="Create Collection" class="w-full h-min mx-2 mt-2">
            {#snippet button()}
                <div class="flex flex-row flex-1 p-2 items-center gap-2 transition ease-in-out duration-75 rounded select-none hover:bg-emerald-800 hover:cursor-pointer">
                    <StackPlus size={24}/>
                    <p>Create</p>
                </div>
            {/snippet}
            {#snippet body()}
                <Tabs.Root value="new">
                    <Tabs.List>
                        <Tabs.Trigger value="new">New</Tabs.Trigger>
                        <Tabs.Trigger value="import">Import</Tabs.Trigger>
                    </Tabs.List>
                    <Tabs.Content value="new">
                        {@render newTab()}
                    </Tabs.Content>
                    <Tabs.Content value="import">
                        {@render importTab()}
                    </Tabs.Content>
                </Tabs.Root>
            {/snippet}
        </Dialogue>
    </div>
    <div class="mb-2 w-full">
        <SidebarButton url={settingsItem.url}>
            <Icon colour="#FFFFFF" icon={settingsItem.icon}/>
            <p>{settingsItem.display}</p>
        </SidebarButton>
    </div>
</div>
