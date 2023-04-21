/* @refresh reload */
import { render } from "solid-js/web";

import App from "./App";
import { invoke } from "@tauri-apps/api";
import { createResource } from "solid-js";
import { listen } from "@tauri-apps/api/event";

render(() => <App/>,document.getElementById("root") as HTMLElement);

async function departments_fetcher () {
  return (await invoke("list_departments")
    .catch(err => console.log(err))) as Name[]
}


export const employees_names_fetcher = async ({name} : {name : () => string | null}) => {
  return (await invoke("search_employees",{name : name() !== ' ' ? name() : null})) as Name[]
}

export const [departmentsNames,dr] = createResource(departments_fetcher)

listen("update_departments",() => {
  dr.refetch()
})

export type ShiftProblem = {
    id: string,
    shift_id: string,
    writer: Name,
    maintainer: Name,
    machine: Name,
    begin_time: string,
    end_time: string,
}

export type NativeDepartment = {
   id            : string,
   boss_id       : string | null,
   name          : string,
}

export type PermissionsClassified = {
      id        : string,
                 //client backend
      allowed   : [string,string][],
      forbidden : [string,string][],
}

export type Permissions = {
  id                                                    :  String,
  write_department_problem                              :  boolean,
  read_department_problems                              :  boolean,
  modify_department_problems                            :  boolean,
  define_problem                                        :  boolean,
  access_history_department_problems                    :  boolean,
  access_history_all_departments_problems               :  boolean,
  access_history_department_department_problems         :  boolean,
  access_history_all_departments_department_problems    :  boolean,
  access_history_machines                               :  boolean,
  access_history_spare_parts                            :  boolean,
  access_history_employees                              :  boolean,
}

export type Problem = {
    id          : string,
    title       : string,
    description : string
}

export type Employee = {
  id            : string,
  card_id       : number,
  department_id : string,
  first_name    : string,
  middle_name   : string,
  last_name     : string,
  password      : string,
  position      : string,
}

export type Universal = {
    employee : Employee | null,
    shiftId  : string   | null
}

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

export type ProblemDeps = {
    problems  : Name[],
    shiftBegin: string,
    shiftEnd  : string
}
