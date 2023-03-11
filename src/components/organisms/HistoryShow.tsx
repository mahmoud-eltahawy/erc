import { ButtonsOrElement } from "../molecules/buttonsOrElement"
import HistoryDays from "../molecules/historyDays"
import HistoryProblems from "../molecules/historyProblems"

export default function HistoryShow({department_id} : {department_id : string}){

  return (
      <section class="LoginFormContainer">
      {<ButtonsOrElement returnButtonText="العودة لصفحة البحث"
                         buttonElementPairs={() => [
                             ["ابحث عن يوم", () => <HistoryDays department_id={department_id} />],
                             ["ابحث عن قطعة غيار",() => { return <h1>قطعة غيار</h1>}],
                             ["ابحث عن مشكلة", () => <HistoryProblems  department_id={department_id} />],
                             ["ابحث عن ماكينة"   ,() => { return <h1>ماكينة</h1>}]
                           ]}
                         num={[-1]}
                         fun={() => console.log("later")}/>}
      </section>
  )
}
