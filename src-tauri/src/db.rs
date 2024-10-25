use crate::err::GenericError;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
	pub static ref db_conn: Arc<Mutex<SqliteConnection>> =
		Arc::new(Mutex::new(connect_db().unwrap()));
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn get_db_file_path() -> Result<String, GenericError> {
	let mut p = match dirs::data_dir() {
		None => return Err(GenericError::new("Could not find data directory")),
		Some(p) => p.into_os_string(),
	};

	#[cfg(target_os = "windows")]
	p.push("\\dienstplaner.db");

	#[cfg(not(target_os = "windows"))]
	p.push("/dienstplaner.db");

	match p.into_string() {
		Ok(p) => Ok(p),
		Err(_) => Err(GenericError::new("Could not convert path to string")),
	}
}

pub fn connect_db() -> Result<SqliteConnection, GenericError> {
	let mut conn = diesel::SqliteConnection::establish(&get_db_file_path()?)?;
	conn.run_pending_migrations(MIGRATIONS)?;

	Ok(conn)
}
