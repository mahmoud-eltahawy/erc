import { createSignal } from "solid-js";
import { css } from "solid-styled-components";
import { shiftId } from "../../App";
import AddShiftNote from "../molecules/AddShiftNote";
import { ButtonsOrElement } from "../molecules/buttonsOrElement";
import DefineProblem from "../molecules/defineProblem";
import SetShiftEmployees from "../molecules/setShiftEmployees";
import ShiftWrittenShow from "../molecules/shiftWrittenNote";
import { ProblemSaveForm } from "./ProblemForm";

export default function CurrentShiftData(props: { rank: number }) {
  const [last, setLast] = createSignal(-1);

  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <section class={container}>
      <ButtonsOrElement
        rank={props.rank + 1}
        buttonElementPairs={() => [
          [
            "اضافة عطل",
            <ProblemSaveForm
              toggle={() => setLast(0)}
            />,
          ],
          [
            "تعريف مشكلة",
            <DefineProblem
              toggle={() => setLast(1)}
            />,
          ],
          [
            "اضافة ملحوظة",
            <AddShiftNote
              toggle={() => setLast(2)}
            />,
          ],
          ["اليومية", <SetShiftEmployees />],
          [
            "البيانات المسجلة",
            <ShiftWrittenShow
              rank={props.rank + 2}
              shiftId={() => shiftId()!}
            />,
          ],
        ]}
        num={last}
        fun={() => setLast(-1)}
      />
    </section>
  );
}
