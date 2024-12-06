<script lang="ts">
import { Button } from "$lib/components/ui/button/index";
import * as Card from "$lib/components/ui/card/index";
import Save from "~icons/material-symbols/save";
import { WIP } from "../../utils";

function save() {
	WIP();
}
</script>

<Card.Root>
	<Card.Content>
		TODO
	</Card.Content>
	<Card.Footer class="justify-end">
		<Button onclick={save}>
			<Save />
		</Button>
	</Card.Footer>
</Card.Root>