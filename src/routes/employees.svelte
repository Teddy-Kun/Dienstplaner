<script lang="ts">
import * as Table from "$lib/components/ui/table";
import { Button } from "@lib/components/ui/button/index";
import * as Card from "@lib/components/ui/card/index";
import { Checkbox } from "@lib/components/ui/checkbox/index";
import Input from "@lib/components/ui/input/input.svelte";
import { onMount } from "svelte";
import EditIcon from "~icons/material-symbols/edit-square";
import Save from "~icons/material-symbols/save";
import TrashIcon from "~icons/mdi/trash-can";
import { create_employee, delete_employee, get_employees } from "../api";
import { type Employee, WIP } from "../utils";

interface EditableEmployee extends Employee {
	checked: boolean;
}
let employees: EditableEmployee[] = $state([
	{
		id: 1,
		name: "Max Mustermann",
		hours: 40,
		overtime: 0,
		checked: false,
	},
]);

let newEmployees: EditableEmployee[] = $state([]);

function edit() {
	WIP();
}

function deleteEmployee(id: number) {
	delete_employee(id).then(() => getEmployees());
}

function deleteChecked() {
	const promises: Promise<unknown>[] = [];
	for (const employee of employees.filter((employee) => employee.checked))
		promises.push(delete_employee(employee.id));

	newEmployees = newEmployees.filter((employee) => !employee.checked);

	Promise.allSettled(promises).then(getEmployees);
}

function add() {
	newEmployees.push({
		id: (newEmployees.length + 1) * -1,
		name: "",
		hours: 0,
		overtime: 0,
		checked: false,
	});
}

function save() {
	const promises: Promise<unknown>[] = [];

	for (const employee of newEmployees)
		promises.push(
			create_employee(employee.name, employee.hours, employee.overtime),
		);

	Promise.allSettled(promises).then(() => {
		newEmployees = [];
		getEmployees();
	});

	WIP();
}

async function getEmployees() {
	employees = (await get_employees()).map((employee) => ({
		...employee,
		checked: false,
	}));
}

onMount(getEmployees);
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
							<Checkbox bind:checked={employee.checked} />
						</Table.Cell>
						<Table.Cell>{employee.name}</Table.Cell>
						<Table.Cell>{employee.hours}</Table.Cell>
						<Table.Cell>{employee.overtime}</Table.Cell>
						<Table.Cell class="flex justify-end items-center">
							<Button class="mr-1" onclick={edit}>
								<EditIcon style="width: 20px; height: 20px; path: currentColor" />
							</Button>
							<Button onclick={() => deleteEmployee(employee.id)}>
								<TrashIcon />
							</Button>
						</Table.Cell>
					</Table.Row>
				{/each}
				{#each newEmployees as employee}
					<Table.Row>
						<Table.Cell>
							<Checkbox bind:checked={employee.checked} />
						</Table.Cell>
						<Table.Cell>
							<Input bind:value={employee.name} />
						</Table.Cell>
						<Table.Cell>
							<Input bind:value={employee.hours} type="number" min="0" max="168" />
						</Table.Cell>
						<Table.Cell>
							<Input bind:value={employee.overtime} type="number" />
						</Table.Cell>
						<Table.Cell class="flex justify-end items-center">
							<Button class="mr-1" onclick={edit}>
								<EditIcon style="width: 20px; height: 20px; path: currentColor" />
							</Button>
							<Button onclick={() => deleteEmployee(employee.id)}>
								<TrashIcon />
							</Button>
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</Card.Content>
	<Card.Footer>
		<Button onclick={add} class="mr-2 text-2xl">+</Button>
		<Button class="items-center justify-center" onclick={deleteChecked}>
			<TrashIcon class="mr-1" />
			Auswahl
		</Button>
		<div class="w-full"></div>
		<Button onclick={save}>
			<Save />
		</Button>
	</Card.Footer>
</Card.Root>
