import { css } from "solid-styled-components";
import ControllAdmins from "../molecules/controllAdmins";
import ControllDepartments from "../molecules/controllDepartments";
import ControllEmployees from "../molecules/controllEmployee";
import { Show } from "solid-js";
import { employee } from "../../App";
import { ButtonsOrElementLite } from "../molecules/buttonsOrElement";

export default function Controlling({ rank }: { rank: number }) {
  const container = css({
    display: "block",
    fontSize: "18px",
    margin: "2px auto",
    padding: "2px",
  });

  return (
    <section class={container}>
      <Show
        when={employee()?.card_id === 0 ||
          employee()?.position === "SUPER_USER"}
        fallback={
          <div>
            <p>ليس لديك الصلاحيات لفتح صفحة التحكم</p>
            <p>يجب ان تكون مشرف او يكون لديك الحساب الرئيسي</p>
          </div>
        }
      >
        <ButtonsOrElementLite
          rank={rank}
          buttonElementPairs={() => [
            ["تعيين مشرف", <ControllAdmins />],
            ["اعدادات الاقسام", <ControllDepartments rank={rank + 1} />],
            ["صلاحيات الموظفين", <ControllEmployees rank={rank + 1} />],
          ]}
        />
      </Show>
    </section>
  );
}
