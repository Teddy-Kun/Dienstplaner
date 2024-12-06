<script lang="ts">
	import { Button } from "$lib/components/ui/button/index";
	import * as Card from "$lib/components/ui/card/index";
	import Save from "~icons/material-symbols/save";
	import store from "../../store.svelte";
	import { WIP } from "../../utils";
	import * as Select from "./ui/select/index";
	import * as Table from "./ui/table/index";

	function save(): void {
		WIP();
	}
</script>

<Card.Root>
	<Card.Content>
		<Select.Root type="single">
			<Select.Trigger />

			<Select.Content>
				{#each store.employees as employee}
					<Select.Item value="{employee.id.toString()}">{employee.name}</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>

		<Table.Root>

		</Table.Root>
	</Card.Content>
	<Card.Footer class="justify-end">
		<Button onclick={save}>
			<Save />
		</Button>
	</Card.Footer>
</Card.Root>