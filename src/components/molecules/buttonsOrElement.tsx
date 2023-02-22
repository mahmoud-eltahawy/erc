import { Show, createSignal,JSXElement, createEffect } from "solid-js";
import { ToggleButton } from "../atoms/toggleButton";

export function ButtonsOrElement({
  returnButtonText  ,
  buttonElementPairs,
  num,
  fun
  } : {
  returnButtonText   : string,
  buttonElementPairs : [string,JSXElement][],
  num : number[],
  fun : Function}){
  const [emptyPlayGround, setEmptyPlayGround] = createSignal<boolean>(true)
  const [toggleButtons, setToggleButtons] = createSignal<boolean[]>(Array(buttonElementPairs.length).fill(false))

  const toggle = (id : number) => {
    setToggleButtons(buttons => buttons.map((cond,index) => {
        if (index !== id) {
          return false
        }
        if(cond){
          setEmptyPlayGround(true)
          return false
        }
        setEmptyPlayGround(false)
        return true
      })
    )
  }

  createEffect(() => {
    if(num[0] !== -1){
        toggle(num[0])
        fun()
    }
  })

  const tbutton = (index : number) => toggleButtons().at(index)

  return (
    <>
      {buttonElementPairs.map((idc, index) => <>
        <Show when={emptyPlayGround() || tbutton(index)}>
          <ToggleButton
                      tButton={() => tbutton(index)!}
                      defaultCont={returnButtonText}
                      cont={idc[0]}
            toggle={() => toggle(index)}
            />
          </Show>
          <Show when={tbutton(index)}>{idc[1]}</Show>
        </>
        )}
    </>
  )
}
