<script lang="ts">
import * as Table from "$lib/components/ui/table";
import { Button } from "@lib/components/ui/button/index";
import * as Card from "@lib/components/ui/card/index";
import { Checkbox } from "@lib/components/ui/checkbox/index";
import type { Employee } from "../utils";

let employees: Employee[] = $state([
	{
		id: 1,
		name: "Max Mustermann",
		hours: 40,
		overtime: 0,
	},
]);

function edit() {
	console.log("TODO: edit");
}

function deleteEmployee() {
	console.log("TODO: delete");
}

function deleteChecked() {
	console.log("TODO: delete checked");
}

function add() {
	console.log("TODO: add");
}

function save() {
	console.log("TODO: save");
}
</script>

<Card.Root>
	<Card.Content>
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>
						<Checkbox />
					</Table.Head>
					<Table.Head>Name</Table.Head>
					<Table.Head>Stunden Pro Woche</Table.Head>
					<Table.Head>Überstunden</Table.Head>
					<Table.Head>Optionen</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each employees as employee}
					<Table.Row>
						<Table.Cell>
							<Checkbox />
						</Table.Cell>
						<Table.Cell>{employee.name}</Table.Cell>
						<Table.Cell>{employee.hours}</Table.Cell>
						<Table.Cell>{employee.overtime}</Table.Cell>
						<Table.Cell>
							<Button class="mr-1" onclick={edit}>Edit</Button>
							<Button onclick={deleteEmployee}>Delete</Button>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</Card.Content>
	<Card.Footer>
		<Button onclick={add} class="mr-2">Add</Button>
		<Button onclick={deleteChecked}>Delte Checked</Button>
		<div class="w-full"></div>
		<Button onclick={save}>Save</Button>
	</Card.Footer>
</Card.Root>
