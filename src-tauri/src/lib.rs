mod color;
mod db;
mod employee;
mod err;
mod schema;
mod settings;
mod workdays;

use color::ColorSchemeAccent;
use db::connect_db;
use employee::Employee;
use err::GenericError;
use settings::Setting;

#[tauri::command]
async fn get_accent_color() -> Result<ColorSchemeAccent, GenericError> {
	#[cfg(target_os = "windows")]
	return color::get_accent_color_windows();

	#[cfg(target_os = "linux")]
	{
		let setts = ashpd::desktop::settings::Settings::new().await?;
		let accent: ColorSchemeAccent = setts.accent_color().await?.into();

		Ok(accent)
	}

	#[cfg(all(not(target_os = "windows"), not(target_os = "linux")))]
	Err(GenericError::new("Unsupported OS"))
}

#[tauri::command]
fn get_employees() -> Result<Vec<Employee>, GenericError> {
	Employee::get_all()
}

#[tauri::command]
fn create_employee(name: String, hours: i32, overtime: i32) -> Result<Employee, GenericError> {
	Employee::new(name, hours, overtime)
}

#[tauri::command]
fn delete_employee(id: i32) -> Result<(), GenericError> {
	let employee = Employee::get(id)?;
	employee.delete()
}

#[tauri::command]
fn put_employee(id: i32, name: String, hours: i32, overtime: i32) -> Result<(), GenericError> {
	let mut e = Employee::get(id)?;

	e.put_employee(name, hours, overtime)?;

	Ok(())
}

#[tauri::command]
fn set_setting(key: String, value: String) -> Result<(), GenericError> {
	Setting::set(&key, value)
}

#[tauri::command]
fn get_setting(key: String) -> Result<String, GenericError> {
	let s = Setting::get(&key)?;
	Ok(s.value)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	if let Err(e) = connect_db() {
		panic!("{:?}", e);
	}

	tauri::Builder::default()
		.plugin(tauri_plugin_shell::init())
		.invoke_handler(tauri::generate_handler![
			get_accent_color,
			get_employees,
			create_employee,
			delete_employee,
			put_employee,
			set_setting,
			get_setting
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
