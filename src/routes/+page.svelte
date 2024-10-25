<script lang="ts">
import Button from "$lib/components/ui/button/button.svelte";
import * as Tabs from "$lib/components/ui/tabs/index.js";
import { invoke } from "@tauri-apps/api/core";
import { toggleMode } from "mode-watcher";

let name = $state("Test");
let greetMsg = $state("");

async function greet(event: Event) {
	event.preventDefault();
	// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
	greetMsg = await invoke("greet", { name });

	console.log(greetMsg);
}
</script>

<main class="container bg-background m-2 flex flex-col items-center w-[100vw] h-[100vh] border-2 border-red-500">
	<Tabs.Root>
		<Tabs.List class="grid w-full grid-cols-2">
			<Tabs.Trigger value="account">Account</Tabs.Trigger>
			<Tabs.Trigger value="password">Password</Tabs.Trigger>
		</Tabs.List>
	</Tabs.Root>
	<h1>Welcome to Tauri + Svelte</h1>

	<Button on:click={toggleMode}>Dark</Button>
	<Button on:click={greet}>Greet</Button>
</main>