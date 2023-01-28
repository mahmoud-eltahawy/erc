import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { EmployeeProvider } from "./employeeProvider";
import "./style.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <EmployeeProvider>
      <App/>
    </EmployeeProvider>
  </React.StrictMode>
);


export type Employee = {
  id            : string,
  card_id       : string,
  department_id : string,
  first_name    : string,
  middle_name   : string,
  last_name     : string,
  password      : string,
  position      : string,
}

export type Name = {
  id   : string,
  name : string
}

export type ProblemDeps = {
    machines  : Name[],
    employees : Name[],
    spareParts: Name[],
    problems  : Name[],
    shiftBegin: string,
    shiftEnd  : string
}
