-- Your SQL goes here
CREATE TABLE workdays(
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	start INTEGER NOT NULL,
	end INTEGER NOT NULL,
	employee_id INTEGER NOT NULL,
	holiday INTEGER NOT NULL,
	FOREIGN KEY (employee_id) REFERENCES employees(id)
);