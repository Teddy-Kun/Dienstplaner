use crate::err::GenericError;
use crate::schema::settings::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Setting {
	id: Option<i32>,
	pub skey: String,
	pub value: String,
}

const ALLOWED_KEYS: [&str; 1] = ["theme"];

impl Setting {
	pub fn get_all() -> Result<Vec<Setting>, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		let res = settings.load::<Setting>(&mut *conn)?;
		Ok(res)
	}

	pub fn get(key: &str) -> Result<Setting, GenericError> {
		if !ALLOWED_KEYS.contains(&key) {
			return Err(GenericError::new("Invalid key"));
		}

		let mut conn = crate::db::db_conn.lock()?;
		let res = settings.filter(skey.eq(key)).first(&mut *conn)?;
		Ok(res)
	}

	pub fn update(&mut self, _value: String) -> Result<(), GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		diesel::update(settings)
			.filter(id.eq(self.id))
			.set(value.eq(_value))
			.execute(&mut *conn)?;

		Ok(())
	}

	pub fn set(key: &str, _value: String) -> Result<(), GenericError> {
		if !ALLOWED_KEYS.contains(&key) {
			return Err(GenericError::new("Invalid key"));
		}

		if let Ok(mut s) = Setting::get(key) {
			s.update(_value)?;
			return Ok(());
		}

		let mut conn = crate::db::db_conn.lock()?;
		diesel::insert_into(settings)
			.values(Setting {
				id: None,
				skey: key.to_string(),
				value: _value,
			})
			.execute(&mut *conn)?;
		Ok(())
	}
}
