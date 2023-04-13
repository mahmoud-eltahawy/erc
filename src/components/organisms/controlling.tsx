import { css } from "solid-styled-components"
import ControllAdmins from "../molecules/controllAdmins"
import ControllDepartments from "../molecules/controllDepartments"
import ControllEmployees from "../molecules/controllEmployee"
import { Show } from "solid-js"
import { employee } from "../../App"
import { ButtonsOrElementLite } from "../molecules/buttonsOrElement"

export default function Controlling(){
  const container = css({
    display: "block",
    fontSize: "18px",
    border: "solid 3px",
    margin: "2px auto",
    padding: "2px",
  })

  return (
    <section class={container}>
      <Show
	      when={
          employee()?.card_id  === 0 ||
          employee()?.position === 'SUPER_USER'
        }
        fallback={<div>
            <p>ليس لديك الصلاحيات لفتح صفحة التحكم</p>
            <p>يجب ان تكون مشرف او يكون لديك الحساب الرئيسي</p>
          </div>}>
            <ButtonsOrElementLite
               returnButtonText="العودة لصفحة التحكم"
               buttonElementPairs={() => [
                   ["تعيين مشرف",  <ControllAdmins/>],
                   ["اعدادات الاقسام",  <ControllDepartments/>],
                   ["صلاحيات الموظفين",  <ControllEmployees/>],
                 ]}/>
      </Show>
      </section>
  )
}
