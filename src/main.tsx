import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { EmployeeAndShiftIDProvider } from "./employeeProvider";
import "./style.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <EmployeeAndShiftIDProvider>
      <App/>
    </EmployeeAndShiftIDProvider>
  </React.StrictMode>
);

export type ShiftProblem = {
  id                : string,
  shiftId           : string,
  writer            : Employee,
  maintainer        : Employee,
  machine           : Machine,
  beginTime         : string,
  endTime           : string,
  problems          : Problem[],
  spareParts        : SparePart[] | null,
  note              : Note | null
}

export type ShiftProblemMini = {
  id                     : string,
  shift_id                : string,
  writer_id              : string,
  maintainer_id          : string,
  machine_id             : string,
  begin_time             : string,
  end_time               : string,
  problems_ids           : string[],
  spare_parts_ids        : string[] | null,
  note                   : Note | null
}

export type Problem = {
    id          : string,
    title       : string,
    description : string
}

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

export type EmployeeAndShiftID = [Employee,string]

export type Name = {
  id   : string,
  name : string
}
export type Note = {
  id      : string,
  content : string
}

export type Machine = {
  id   : string,
  name : string
}

export type SparePart = {
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
