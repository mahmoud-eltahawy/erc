import { invoke } from "@tauri-apps/api"
import { createEffect, createResource, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import { departmentsNames } from "../.."
import { ButtonsOrElement } from "./buttonsOrElement"
import HistoryShiftProblems from "./historyShiftProblems"

type Day = {
    date   : [string,string,string],
    shifts : [string,string][]
}

export default function HistoryDays({department_id} : {department_id : string | null}){
  const [dates,setDates] = createStore<[string | null,string | null]>([null,null])

  const dateContainer = css({
    display: "inline-block",
    width: "40%",
    paddingLeft: "10px",
    paddingRight: "10px",
    marginTop: "20px",
    marginRight: "3%",
    marginLeft: "3%",
  })

  const dateInput = css({
    display: "inline-block",
    fontSize: "20px",
    margin: ".1em auto",
    width: "60%",
    backgroundColor:"lightyellow",
    borderRadius: "20px",
  })

  const dateLabel = css({
  display: "inline-block",
  width: "35%",
  padding: ".1em",
  margin: ".1em auto",
  })

  return (
    <section>
      <div class={dateContainer}>
        <input value={dates[1]!}
               onChange={e => setDates([dates[0],e.currentTarget.value])}
               class={dateInput}
               type="date"
               required/>
        <label class={dateLabel}><h4>وقت النهاية</h4></label>
      </div>
      <div class={dateContainer}>
        <input value={dates[0]!}
               onChange={e => setDates([e.currentTarget.value,dates[1]])}
               class={dateInput}
               type="date"
               required/>
        <label class={dateLabel}><h4>وقت البداية</h4></label>
      </div>
      <Show
          when={department_id}
          fallback={<ShowAllHistory dates={() => dates}/>}>
        <ShowHistory
          departmentId={department_id!}
          dates={dates}/>
      </Show>
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

function ShowAllHistory({dates} : {dates :() => [string | null,string | null]}){
    return (
        <ButtonsOrElement
              returnButtonText="الرجوع الي الاقسام"
              buttonElementPairs={() => (departmentsNames() || [])
                .map(d => [d.name, <ShowHistory
                                      departmentId={d.id}
                                      dates={dates()}/>])}
              num={[-1]}
              fun={() => console.log("later")}/>
    )
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

  const container = css({
   display: "block",
   fontSize: "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
      <div class={container}>
        <Show when={days()}>
          <ButtonsOrElement
            buttonElementPairs={() => (days() || []).
                map(x => [x.date.join(" / "), () => <Shifts shifts={() => x.shifts}/>])}
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
