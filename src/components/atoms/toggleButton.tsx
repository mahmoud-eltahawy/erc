export function ToggleButton({
    toggle,
    cont,
    defaultCont,
    tButton
    } : {
    toggle          : Function,
    cont            : string,
    defaultCont     : string,
    tButton         : Function
}){
  return (
    <button onClick={() => toggle()}>
        {tButton() ? defaultCont : cont}
    </button>
  )
}
