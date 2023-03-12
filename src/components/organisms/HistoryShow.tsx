import { ButtonsOrElement } from "../molecules/buttonsOrElement"
import HistoryDays from "../molecules/historyDays"
import HistoryMachines from "../molecules/historyMachines"
import HistoryParts from "../molecules/historyParts"
import HistoryProblems from "../molecules/historyProblems"

export default function HistoryShow({department_id} : {department_id : string}){

  return (
      <section class="LoginFormContainer">
      {<ButtonsOrElement returnButtonText="العودة لصفحة البحث"
                         buttonElementPairs={() => [
                             ["ابحث عن يوم"     , () => <HistoryDays department_id={department_id} />],
                             ["ابحث عن قطعة غيار", () => <HistoryParts/>],
                             ["ابحث عن مشكلة"   , () => <HistoryProblems  department_id={department_id} />],
                             ["ابحث عن ماكينة"   , () => <HistoryMachines/>]
                           ]}
                         num={[-1]}
                         fun={() => console.log("later")}/>}
      </section>
  )
}
