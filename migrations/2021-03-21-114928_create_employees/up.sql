-- Your SQL goes here


CREATE TABLE Employees (
	emp_id INTEGER PRIMARY KEY,
	emp_name TEXT NOT NULL,
	emp_desig TEXT,
	emp_salary TEXT
);


CREATE TABLE Projects (
	proj_id INTEGER PRIMARY KEY,
	proj_title TEXT NOT NULL,
	proj_cost INTEGER NOT NULL,
	proj_duration INTEGER
);


CREATE TABLE Employee_projects ( 
   emp_id INTEGER,
   proj_id INTEGER,
   PRIMARY KEY (emp_id, proj_id),
   FOREIGN KEY (emp_id) 
      REFERENCES Employees (emp_id) 
         ON DELETE CASCADE 
         ON UPDATE NO ACTION,
   FOREIGN KEY (proj_id) 
      REFERENCES Projects (proj_id) 
         ON DELETE CASCADE 
         ON UPDATE NO ACTION
);


