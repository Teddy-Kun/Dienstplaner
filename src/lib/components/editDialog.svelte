<script lang="ts">
import Button from "$lib/components/ui/button/button.svelte";
import * as Dialog from "$lib/components/ui/dialog/index";
import Input from "$lib/components/ui/input/input.svelte";
import { onMount } from "svelte";
import Save from "~icons/material-symbols/save";
import { put_employee } from "../../api";
import type { Employee } from "../../utils";

interface Props {
	open: boolean;
	employee: Employee;
}
let {
	open = $bindable(false),
	employee = $bindable({ id: 0, name: "", hours: 0, overtime: 0 }),
}: Props = $props();

let afterEdit: Employee = $state({ id: 0, name: "", hours: 0, overtime: 0 });

function save() {
	put_employee(employee).then(() => {
		open = false;
	});
}

let waitForProp = $state(true);

$effect(() => {
	if (waitForProp) {
		waitForProp = false;
		afterEdit = JSON.parse(JSON.stringify(employee));
	}
});

onMount(() => {
	waitForProp = true;
});
</script>

<Dialog.Root bind:open={open}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Bearbeitung von {employee.name}</Dialog.Title>
		</Dialog.Header>

		<Input bind:value={afterEdit.name} />
		<Input bind:value={afterEdit.hours} type="number" />
		<Input bind:value={afterEdit.overtime} type="number" />

		<Dialog.Footer class="justify-end">
			<Button onclick={save}>
				<Save />
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>