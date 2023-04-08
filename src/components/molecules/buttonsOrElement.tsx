import { Show, createSignal,JSXElement, createEffect, For } from "solid-js";
import { ToggleButton } from "../atoms/toggleButton";

export function ButtonsOrElement({
  returnButtonText  ,
  buttonElementPairs,
  num,
  fun
  } : {
  returnButtonText   : string,
  buttonElementPairs :() => (string | (() => JSXElement))[][],
  num : number[],
  fun : Function}){

  const [buttonIndex, setButtonIndex] = createSignal(-1)

  const toggle = (id : number) => {
      if(buttonIndex() === id){
          setButtonIndex(-1)
      } else {
          setButtonIndex(id)
      }
  }

  createEffect(() => {
    if(num[0] !== -1){
      toggle(num[0])
      fun()
    }
  })

  const isChosen = (index : number) =>  buttonIndex() === index

  return (
    <For each={buttonElementPairs()}>
      {(item,index) => <>
        <Show when={buttonIndex() === -1 || isChosen(index())}>
          <ToggleButton
            tButton={() => isChosen(index())}
            defaultCont={returnButtonText}
            cont={item[0] as string}
            toggle={() => toggle(index())}/>
          </Show>
        <Show when={isChosen(index())}>{item[1]}</Show>
      </>}
    </For>
  )
}
