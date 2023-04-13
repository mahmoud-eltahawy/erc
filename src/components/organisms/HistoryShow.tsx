import { css } from "solid-styled-components"
import { ButtonsOrElementLite } from "../molecules/buttonsOrElement"
import HistoryDays from "../molecules/historyDays"
import HistoryEmployees from "../molecules/historyEmployees"
import HistoryMachines from "../molecules/historyMachines"
import HistoryParts from "../molecules/historyParts"
import HistoryProblems from "../molecules/historyProblems"

export default function HistoryShow(){

  const container = css({
   display: "block",
   fontSize: "18px",
   border: "solid 3px",
   margin: "2px auto",
   padding: "2px",
  })

  return (
    <section class={container}>
        <ButtonsOrElementLite
          returnButtonText="العودة لصفحة البحث"
          buttonElementPairs={() => [
                ["ابحث عن يوم", <HistoryDays/>],
                ["ابحث عن مشكلة", <HistoryProblems/>],
                ["ابحث عن قطعة غيار", <HistoryParts/>],
                ["ابحث عن ماكينة"   , <HistoryMachines/>],
                ["ابحث عن موظف"   , <HistoryEmployees/>]
            ]
          }/>
    </section>
  )
}
