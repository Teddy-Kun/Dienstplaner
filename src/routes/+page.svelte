<script lang="ts">
import Button from "$lib/components/ui/button/button.svelte";
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

<main class="container bg-background">
	<h1>Welcome to Tauri + Svelte</h1>

	<Button on:click={toggleMode}>Dark</Button>
	<Button on:click={greet}>Greet</Button>
</main>