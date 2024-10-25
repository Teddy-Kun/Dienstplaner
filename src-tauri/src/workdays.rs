use crate::err::GenericError;
use crate::schema::workdays::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::workdays)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Workday {
	id: Option<i32>,
	start: i32,
	end: i32,
	employee_id: i32,
	holiday: i32,
}

impl Workday {
	pub fn new(s: i32, e: i32, e_id: i32, h: bool) -> Result<Workday, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;

		let res = diesel::insert_into(crate::schema::workdays::table)
			.values(Workday {
				id: None,
				start: s,
				end: e,
				employee_id: e_id,
				holiday: if h { 1 } else { 0 },
			})
			.returning(Workday::as_returning())
			.get_result(&mut *conn)?;

		Ok(res)
	}

	pub fn get_all() -> Result<Vec<Workday>, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		let res = workdays.load::<Workday>(&mut *conn)?;
		Ok(res)
	}

	pub fn get_employee_workdays(
		e_id: i32,
		_start: Option<i32>,
		_end: Option<i32>,
	) -> Result<Vec<Workday>, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;

		let start_filter = match _start {
			Some(s) => start.gt(s),
			None => start.gt(0),
		};

		let end_filter = match _end {
			Some(e) => end.lt(e),
			None => end.lt(i32::MAX),
		};

		let res = workdays
			.filter(employee_id.eq(e_id))
			.filter(start_filter)
			.filter(end_filter)
			.load::<Workday>(&mut *conn)?;
		Ok(res)
	}

	pub fn delete(&self) -> Result<usize, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		let res = diesel::delete(workdays.filter(id.eq(self.id))).execute(&mut *conn)?;
		Ok(res)
	}
}
