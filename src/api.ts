import { core } from "@tauri-apps/api";
import { toast } from "svelte-sonner";
import {
	type ColorSchemeAccent,
	type Employee,
	default_windows_accent,
	setPrimaryColor,
} from "./utils";

function apiError(err: unknown) {
	toast.error(err as string);
}

export async function create_employee(
	name: string,
	hours: number,
	overtime: number,
): Promise<Employee | null> {
	try {
		const resp = (await core.invoke("create_employee", {
			name,
			hours: Number(hours),
			overtime: Number(overtime),
		})) as Employee;

		return resp;
	} catch (err) {
		apiError(err);

		return null;
	}
}

export async function get_employees(): Promise<Employee[]> {
	try {
		const resp = (await core.invoke("get_employees")) as Employee[];
		return resp;
	} catch (err) {
		apiError(err);
		return [];
	}
}

export async function delete_employee(id: number) {
	try {
		await core.invoke("delete_employee", { id });
	} catch (err) {
		apiError(err);
	}
}

export async function put_employee(
	id: number,
	name: string,
	hours: number,
	overtime: number,
) {
	try {
		await core.invoke("put_employee", {
			id,
			name,
			hours: Number(hours),
			overtime: Number(overtime),
		});
	} catch (err) {
		apiError(err);
	}
}

export async function set_theme(theme: "light" | "dark" | "auto") {
	try {
		await core.invoke("set_setting", { key: "theme", value: theme });
	} catch (err) {
		apiError(err);
	}
}

export async function get_theme(
	suppressError?: boolean,
): Promise<"light" | "dark" | "auto"> {
	try {
		const s = await core.invoke("get_setting", { key: "theme" });
		return s as "light" | "dark" | "auto";
	} catch (err) {
		if (!suppressError) apiError(err);
		return "auto";
	}
}

export async function getAccentColor(
	suppressError?: boolean,
): Promise<ColorSchemeAccent> {
	try {
		const resp = (await core.invoke("get_accent_color")) as ColorSchemeAccent;

		setPrimaryColor(resp);

		return resp;
	} catch (err) {
		if (!suppressError) apiError(err);

		setPrimaryColor(default_windows_accent);

		return structuredClone(default_windows_accent);
	}
}
