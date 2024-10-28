import { core } from "@tauri-apps/api";
import { toast } from "svelte-sonner";

export interface ColorSchemeAccent {
	hue: number;
	saturation: number;
	luminance: number;
	hex_code: string;
}

export interface Employee {
	id: number;
	name: string;
	hours: number;
	overtime: number;
}

const default_windows_accent: ColorSchemeAccent = {
	hue: 207,
	saturation: 100,
	luminance: 42,
	hex_code: "#0077d6",
};

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

function getHslString(color: ColorSchemeAccent): string {
	return `${color.hue} ${color.saturation}% ${color.luminance}%`;
}

function setPrimaryColor(color: ColorSchemeAccent) {
	const accent = structuredClone(color);
	accent.luminance += 15;
	document.documentElement.style.setProperty("--primary", getHslString(color));
	document.documentElement.style.setProperty("--accent", getHslString(accent));
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

		setPrimaryColor(default_windows_accent)

		return structuredClone(default_windows_accent);
	}
}
