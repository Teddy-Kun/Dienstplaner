<script lang="ts">
import { Button } from "$lib/components/ui/button/index";
import * as Card from "$lib/components/ui/card/index";
import { Checkbox } from "$lib/components/ui/checkbox/index";
import Input from "$lib/components/ui/input/input.svelte";
import * as Table from "$lib/components/ui/table";
import { onMount } from "svelte";
import EditIcon from "~icons/material-symbols/edit-square";
import Save from "~icons/material-symbols/save";
import TrashIcon from "~icons/mdi/trash-can";
import { create_employee, delete_employee, get_employees } from "../api";
import { type Employee } from "../utils";
import EditDialog from "./editDialog.svelte";

interface EditableEmployee extends Employee {
	checked: boolean;
}
let employees: EditableEmployee[] = $state([]);
let newEmployees: EditableEmployee[] = $state([]);

let editing: Employee = $state({ id: 0, name: "", hours: 0, overtime: 0 });
let allChecked: boolean | "indeterminate" = $state(false);
let openEdit: boolean = $state(false);

$effect(() => {
	if (allChecked === "indeterminate") return;

	for (const employee of employees) employee.checked = allChecked;

	for (const employee of newEmployees) employee.checked = allChecked;
});

function check() {
	console.log("checking...");

	if (employees.every((e) => e.checked) && newEmployees.every((e) => e.checked))
		allChecked = true;
	else if (
		employees.every((e) => !e.checked) &&
		newEmployees.every((e) => !e.checked)
	)
		allChecked = false;
	else allChecked = "indeterminate";
}

function edit(index: number) {
	editing = {
		id: employees[index].id,
		name: employees[index].name,
		hours: employees[index].hours,
		overtime: employees[index].overtime,
	};
	openEdit = true;
}

$effect(() => {
	if (!openEdit) {
		getEmployees();
		editing = { id: 0, name: "", hours: 0, overtime: 0 };
	}
});

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
}

async function getEmployees() {
	employees = (await get_employees()).map((employee) => ({
		...employee,
		checked: false,
		editing: false,
	}));
}

onMount(getEmployees);
</script>

<EditDialog bind:open={openEdit} bind:employee={editing} />
<Card.Root>
	<Card.Content>
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>
						<Checkbox bind:checked={allChecked} />
					</Table.Head>
					<Table.Head>Name</Table.Head>
					<Table.Head>Stunden Pro Woche</Table.Head>
					<Table.Head>Überstunden</Table.Head>
					<Table.Head>Optionen</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each employees as employee, i}
					<Table.Row>
						<Table.Cell>
							<Checkbox bind:checked={employee.checked} onCheckedChange={check} />
						</Table.Cell>
						<Table.Cell>{employee.name}</Table.Cell>
						<Table.Cell>{employee.hours}</Table.Cell>
						<Table.Cell>{employee.overtime}</Table.Cell>
						<Table.Cell class="flex justify-end items-center">
							<Button class="mr-1" onclick={() => edit(i)}>
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
							<Checkbox bind:checked={employee.checked} onCheckedChange={check} />
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
							<Button onclick={() => {newEmployees = newEmployees.filter((e) => e !== employee)}}>
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
		<Button disabled={allChecked === false} class="items-center justify-center" onclick={deleteChecked}>
			<TrashIcon class="mr-1" />
			Auswahl
		</Button>
		<div class="w-full"></div>
		<Button disabled={newEmployees.length === 0} onclick={save}>
			<Save />
		</Button>
	</Card.Footer>
</Card.Root>
