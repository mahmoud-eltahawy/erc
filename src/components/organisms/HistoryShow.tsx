import { JSXElement } from "solid-js"
import { css } from "solid-styled-components"
import { HistoryPermissions } from "../.."
import { ButtonsOrElement } from "../molecules/buttonsOrElement"
import HistoryDays from "../molecules/historyDays"
import HistoryEmployees from "../molecules/historyEmployees"
import HistoryMachines from "../molecules/historyMachines"
import HistoryParts from "../molecules/historyParts"
import HistoryProblems from "../molecules/historyProblems"

export default function HistoryShow({
    department_id,
    permissions
    } : {
    department_id : string,
    permissions   : () => HistoryPermissions
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
      {<ButtonsOrElement returnButtonText="العودة لصفحة البحث"
                         buttonElementPairs={() => {
                           const {
                              access_history_all_departments_department_problems,
                              access_history_all_departments_problems,
                              access_history_department_department_problems,
                              access_history_department_problems,
                              access_history_employees,
                              access_history_machines,
                              access_history_spare_parts
                           } = permissions()

                           const pairs : [string,JSXElement][] = []

                           if(access_history_all_departments_department_problems){
                               pairs.unshift(["ابحث عن يوم",
                                              () => <HistoryDays department_id={null}/>])
                           } else if(access_history_department_department_problems){
                               pairs.unshift(["ابحث عن يوم",
                                              () => <HistoryDays department_id={department_id}/>])
                           }

                           if(access_history_all_departments_problems){
                               pairs.unshift(["ابحث عن مشكلة",
                                              () => <HistoryProblems department_id={null}/>])
                           } else if(access_history_department_problems){
                               pairs.unshift(["ابحث عن مشكلة",
                                              () => <HistoryProblems department_id={department_id}/>])
                           }

                           if(access_history_spare_parts){
                               pairs.unshift(["ابحث عن قطعة غيار", () => <HistoryParts/>])
                           }

                           if(access_history_machines){
                               pairs.unshift(["ابحث عن ماكينة"   , () => <HistoryMachines/>])
                           }

                           if(access_history_employees){
                               pairs.unshift(["ابحث عن موظف"   , () => <HistoryEmployees />])
                           }

                           return pairs
                         }}
                         num={[-1]}
                         fun={() => console.log("later")}/>}
      </section>
  )
}
