import { css } from "solid-styled-components";
import { ButtonsOrElementLite } from "../molecules/buttonsOrElement";
import HistoryDays from "../molecules/historyDays";
import HistoryEmployees from "../molecules/historyEmployees";
import HistoryMachines from "../molecules/historyMachines";
import HistoryParts from "../molecules/historyParts";
import HistoryProblems from "../molecules/historyProblems";

export default function HistoryShow(props: { rank: number }) {
  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <section class={container}>
      <ButtonsOrElementLite
        rank={props.rank}
        buttonElementPairs={() => [
          ["ابحث عن يوم", <HistoryDays rank={props.rank + 1} />],
          ["ابحث عن مشكلة", <HistoryProblems rank={props.rank + 1} />],
          ["ابحث عن قطعة غيار", <HistoryParts rank={props.rank + 1} />],
          ["ابحث عن ماكينة", <HistoryMachines rank={props.rank + 1} />],
          ["ابحث عن موظف", <HistoryEmployees rank={props.rank + 1} />],
        ]}
      />
    </section>
  );
}
