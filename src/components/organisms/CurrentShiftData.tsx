import { createStore } from "solid-js/store"
import { css } from "solid-styled-components"
import { shiftId } from "../../App"
import AddShiftNote from "../molecules/AddShiftNote"
import { ButtonsOrElement } from "../molecules/buttonsOrElement"
import DefineProblem from "../molecules/defineProblem"
import SetShiftEmployees from "../molecules/setShiftEmployees"
import ShiftWrittenShow from "../molecules/shiftWrittenNote"
import ProblemForm from "./ProblemForm"

export default function CurrentShiftData() {
  const [last,setLast] = createStore([-1])

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
        buttonElementPairs={() => [
            ["اضافة عطل",<ProblemForm
                                 toggle={() => setLast([0])}/>],
            ["تعريف مشكلة",<DefineProblem
                                  toggle={() => setLast([1])}/>],
            ["اضافة ملحوظة",<AddShiftNote
                                  toggle={() => setLast([2])}/>],
            ["اليومية",<SetShiftEmployees />],
            ["البيانات المسجلة",<ShiftWrittenShow shiftId={() => shiftId()!} />],
          ]
        }
        num={last}
        fun={() => setLast([-1])}/>
    </section>
  )
}
