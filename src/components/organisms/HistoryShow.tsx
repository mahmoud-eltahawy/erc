import { css } from "solid-styled-components";
import { ButtonsOrElementLite } from "../molecules/buttonsOrElement";
import HistoryDays from "../molecules/historyDays";
import HistoryEmployees from "../molecules/historyEmployees";
import HistoryMachines from "../molecules/historyMachines";
import HistoryParts from "../molecules/historyParts";
import HistoryProblems from "../molecules/historyProblems";

export default function HistoryShow({ rank }: { rank: number }) {
  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <section class={container}>
      <ButtonsOrElementLite
        rank={rank}
        buttonElementPairs={() => [
          ["ابحث عن يوم", <HistoryDays rank={rank + 1} />],
          ["ابحث عن مشكلة", <HistoryProblems rank={rank + 1} />],
          ["ابحث عن قطعة غيار", <HistoryParts rank={rank + 1} />],
          ["ابحث عن ماكينة", <HistoryMachines rank={rank + 1} />],
          ["ابحث عن موظف", <HistoryEmployees rank={rank + 1} />],
        ]}
      />
    </section>
  );
}
