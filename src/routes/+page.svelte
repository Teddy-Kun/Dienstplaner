<script lang="ts">
	import Employees from "$lib/components/employees.svelte";
	import Settings from "$lib/components/settings.svelte";
	import Time from "$lib/components/time.svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import * as Tabs from "$lib/components/ui/tabs/index.js";
	import { getAccentColor } from "../api";

	let header = $state("time");

	$inspect(header).with(() => console.log("header", header));

	getAccentColor(true);
</script>

<Toaster />
<main class="container">
	<Tabs.Root bind:value={header}>
		<Tabs.List class="grid w-full grid-cols-3 my-2">
			<Tabs.Trigger value="time">Dienstplan</Tabs.Trigger>
			<Tabs.Trigger value="employee">Mitarbeiter</Tabs.Trigger>
			<Tabs.Trigger value="settings">Einstellungen</Tabs.Trigger>
		</Tabs.List>

		<Tabs.Content value="time">
			<Time />
		</Tabs.Content>
		<Tabs.Content value="employee">
			<Employees />
		</Tabs.Content>
		<Tabs.Content value="settings">
			<Settings />
		</Tabs.Content>
	</Tabs.Root>
</main>