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
  shift_id               : string,
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

export type SparePart = {
  id   : string,
  name : string
}

export type ProblemDeps = {
    problems  : Name[],
    shiftBegin: string,
    shiftEnd  : string
}

export async function shiftProblemFromMinimal(mp : ShiftProblemMini) : Promise<ShiftProblem> {
  const problems : Problem[] = []
  for(let j =0; j < mp.problems_ids.length; j++){
    problems.push(await invoke('get_problem_by_id',{id : mp.problems_ids[j]}) as Problem)
  }

  const spareParts : SparePart[] | null = mp.spare_parts_ids ? [] : null
  if(mp.spare_parts_ids){
    for(let j =0; j < mp.spare_parts_ids.length; j++){
      spareParts!.push(await invoke('get_spare_part_by_id',{id : mp.spare_parts_ids[j]}) as SparePart)
    }
  }

  return {
      id          : mp.id,
      shiftId     : mp.shift_id,
      note        : mp.note,
      writer      : await invoke('get_employee_by_id',{id : mp.writer_id})     as Employee,
      maintainer  : await invoke('get_employee_by_id',{id : mp.maintainer_id}) as Employee,
      machine     : await invoke('get_machine_by_id' ,{id : mp.machine_id})    as Machine,
      beginTime   : mp.begin_time,
      endTime     : mp.end_time,
      problems    : problems,
      spareParts  : spareParts
  }
}

export async function problemsFetcher(shiftId : string){
  const sp = await invoke('get_current_shift_problems',{shiftId}) as ShiftProblemMini[]
  let arr : ShiftProblem[] = []
  for(let i = 0 ; i < sp.length ; i++){
      arr.push(await shiftProblemFromMinimal(sp[i]))
  }
  return arr
}
