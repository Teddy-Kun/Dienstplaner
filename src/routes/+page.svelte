<script lang="ts">
import { Toaster } from "$lib/components/ui/sonner";
import * as Tabs from "$lib/components/ui/tabs/index.js";
import { getAccentColor } from "../api";

getAccentColor(true);
</script>

<Toaster />
<main class="container">
	<Tabs.Root value="time">
		<Tabs.List class="grid w-full grid-cols-3 my-2">
			<Tabs.Trigger value="time">Dienstplan</Tabs.Trigger>
			<Tabs.Trigger value="employee">Mitarbeiter</Tabs.Trigger>
			<Tabs.Trigger value="settings">Einstellungen</Tabs.Trigger>
		</Tabs.List>
	</Tabs.Root>
</main>