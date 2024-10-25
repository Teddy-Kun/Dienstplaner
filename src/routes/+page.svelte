<script lang="ts">
import Button from "$lib/components/ui/button/button.svelte";
import { Toaster } from "$lib/components/ui/sonner";
import * as Tabs from "$lib/components/ui/tabs/index.js";
import { invoke } from "@tauri-apps/api/core";
import { toggleMode } from "mode-watcher";
import { toast } from "svelte-sonner";

let name = $state("Test");
let greetMsg = $state("");

async function greet(event: Event) {
	event.preventDefault();
	// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
	greetMsg = await invoke("greet", { name });

	toast.success(greetMsg);
}
</script>

<Toaster />

<main class="container">
	<Tabs.Root>
		<Tabs.List class="grid w-full grid-cols-3 my-2">
			<Tabs.Trigger value="time">Dienstplan</Tabs.Trigger>
			<Tabs.Trigger value="employee">Mitarbeiter</Tabs.Trigger>
			<Tabs.Trigger value="settings">Einstellungen</Tabs.Trigger>
		</Tabs.List>
	</Tabs.Root>

	<Button on:click={toggleMode}>Dark</Button>
	<Button on:click={greet}>Greet</Button>
	<Button on:click={() => toast.error("Test")}>Test</Button>
</main>