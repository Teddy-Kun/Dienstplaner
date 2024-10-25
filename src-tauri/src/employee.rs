use crate::err::GenericError;
use crate::schema::employees::dsl::*;
use crate::workdays::Workday;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Selectable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::employees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
	id: Option<i32>,
	name: String,
	hours: i32,
	overtime: i32,
}

impl Employee {
	pub fn new(_name: String, _hours: i32, _overtime: i32) -> Result<Employee, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		let res = diesel::insert_into(employees)
			.values(Employee {
				id: None,
				name: _name,
				hours: _hours,
				overtime: _overtime,
			})
			.returning(Employee::as_returning())
			.get_result(&mut *conn)?;

		Ok(res)
	}

	pub fn get(_id: i32) -> Result<Employee, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		let res = employees.filter(id.eq(_id)).first(&mut *conn)?;
		Ok(res)
	}

	pub fn get_all() -> Result<Vec<Employee>, GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		let res = employees.load::<Employee>(&mut *conn)?;
		Ok(res)
	}

	pub fn put_employee(
		&mut self,
		_name: String,
		_hours: i32,
		_overtime: i32,
	) -> Result<(), GenericError> {
		let mut conn = crate::db::db_conn.lock()?;
		diesel::update(employees)
			.filter(id.eq(self.id.unwrap()))
			.set((name.eq(&_name), hours.eq(_hours), overtime.eq(_overtime)))
			.execute(&mut *conn)?;

		self.name = _name;
		self.hours = _hours;
		self.overtime = _overtime;

		Ok(())
	}

	pub fn delete(&self) -> Result<(), GenericError> {
		let employee_workdays = Workday::get_employee_workdays(self.id.unwrap(), None, None)?;

		for wd in employee_workdays {
			wd.delete()?;
		}

		let mut conn = crate::db::db_conn.lock()?;

		diesel::delete(employees)
			.filter(id.eq(self.id.unwrap()))
			.execute(&mut *conn)?;
		Ok(())
	}
}
