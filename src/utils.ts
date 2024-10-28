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

export const default_windows_accent: ColorSchemeAccent = {
	hue: 207,
	saturation: 100,
	luminance: 42,
	hex_code: "#0077d6",
};
Object.freeze(default_windows_accent);

export const black: ColorSchemeAccent = {
	hue: 0,
	saturation: 0,
	luminance: 0,
	hex_code: "#000",
};
Object.freeze(black);

export const white: ColorSchemeAccent = {
	hue: 0,
	saturation: 0,
	luminance: 100,
	hex_code: "#fff",
};
Object.freeze(white);

function getHslString(color: ColorSchemeAccent): string {
	return `${color.hue} ${color.saturation}% ${color.luminance}%`;
}

export function setPrimaryColor(color: ColorSchemeAccent) {
	const accent = structuredClone(color);
	accent.luminance += 15;
	document.documentElement.style.setProperty("--primary", getHslString(color));
	document.documentElement.style.setProperty("--accent", getHslString(accent));

	if (accent.luminance > 60) {
		document.documentElement.style.setProperty(
			"--primary-foreground",
			getHslString(black),
		);

		document.documentElement.style.setProperty(
			"--accent-foreground",
			getHslString(black),
		);
	} else {
		document.documentElement.style.setProperty(
			"--primary-foreground",
			getHslString(white),
		);

		document.documentElement.style.setProperty(
			"--accent-foreground",
			getHslString(white),
		);
	}
}
