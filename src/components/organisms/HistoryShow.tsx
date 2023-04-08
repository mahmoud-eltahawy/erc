import { JSXElement } from "solid-js"
import { css } from "solid-styled-components"
import { ButtonsOrElement } from "../molecules/buttonsOrElement"
import HistoryDays from "../molecules/historyDays"
import HistoryEmployees from "../molecules/historyEmployees"
import HistoryMachines from "../molecules/historyMachines"
import HistoryParts from "../molecules/historyParts"
import HistoryProblems from "../molecules/historyProblems"

export default function HistoryShow({
    department_id,
    } : {
    department_id : string,
}){

  const container = css({
   display: "block",
   fontSize: "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
      <section class={container}>
      <ButtonsOrElement
          returnButtonText="العودة لصفحة البحث"
          buttonElementPairs={() => {
            return [
                ["ابحث عن يوم",
                      () => <HistoryDays department_id={department_id}/>],
                ["ابحث عن مشكلة",
                      () => <HistoryProblems department_id={department_id}/>],
                ["ابحث عن قطعة غيار", () => <HistoryParts/>],
                ["ابحث عن ماكينة"   , () => <HistoryMachines/>],
                ["ابحث عن موظف"   , () => <HistoryEmployees />]
            ] as (string | (() => JSXElement))[][]
          }}
          num={[-1]}
          fun={() => console.log("later")}/>
      </section>
  )
}
