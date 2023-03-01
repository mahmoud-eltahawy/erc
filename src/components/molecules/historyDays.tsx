import { invoke } from "@tauri-apps/api"
import { createEffect, createResource, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { ButtonsOrElement } from "./buttonsOrElement"
import HistoryShiftProblems from "./historyShiftProblems"

type Day = {
    date   : [string,string,string],
    shifts : [string,string][]
}

export default function HistoryDays({department_id} : {department_id : string}){
  const [dates,setDates] = createStore<[string | null,string | null]>([null,null])

  return (
    <section>
      <div class={"problemFormTimeBlock"}>
        <input value={dates[1]!}
               onChange={e => setDates([dates[0],e.currentTarget.value])}
               class={"problemFormTimeInput"}
               type="date"
               required/>
        <label class="problemFormTimeLabel"><h4>وقت النهاية</h4></label>
      </div>
      <div class={"problemFormTimeBlock"}>
        <input value={dates[0]!}
               onChange={e => setDates([e.currentTarget.value,dates[1]])}
               class={"problemFormTimeInput"}
               type="date"
               required/>
        <label class={"problemFormTimeLabel"}><h4>وقت البداية</h4></label>
      </div>
      <ShowHistory
          departmentId={department_id}
          dates={dates}/>
    </section>
  )
}

const fetcher = async (args : {
    departmentId  : () => string,
    begin         : () => string | null | undefined,
    end           : () => string | null | undefined
}) => {
    return (await invoke("search_shifts",{departmentId : args.departmentId(),
                                    begin :args.begin(),end : args.end()})) as Day[]
}

function ShowHistory({
    departmentId,
    dates
    } : {
    departmentId : string,
    dates :[string | null,string | null],
    }){
  const [days, { refetch }] = createResource({ departmentId : () => departmentId,
                    begin: () => dates.at(0), end: () => dates.at(1)},fetcher);


  createEffect(() => {
      if(dates[0] && dates[1]){
        refetch()
      }
  })

  return (
      <div class="LoginFormContainer">
        <Show when={days()}>
          <ButtonsOrElement
            buttonElementPairs={() => (days() || []).
                map(x => [x.date.join(" / "), () => <Shifts shifts={() => x.shifts} />])}
            num={[-1]}
            fun={() => console.log("fun")}
            returnButtonText="يوم اخر"/>
        </Show>
      </div>
  )
}

function Shifts({shifts} : {shifts : () => [string,string][]}){
    return (
        <div>
        <ButtonsOrElement
                buttonElementPairs={() => shifts().
                    map(x => [x[1], () => <HistoryShiftProblems shiftId={x[0]} />])}
                num={[-1]}
                fun={() => console.log("fun")}
                returnButtonText="وردية اخري"/>
        </div>
    )
}
