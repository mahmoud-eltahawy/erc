import { invoke } from '@tauri-apps/api/tauri'
import { createResource } from 'solid-js';
import { css } from 'solid-styled-components';

const fetcher = async () => {
  const [order,date] = await invoke('current_shift') as [string,[String, String, String]]
  return [date.join(" / "),order]
}

export default function ShiftIdentity(){
  const [shift,{refetch}] = createResource(fetcher)

  const date  = () => (shift() || []).at(0)
  const order = () => (shift() || []).at(1)

  setInterval(() => refetch(),60000)

  const container = css({
    margin: "0px",
    padding: "0px",
    display: "flex",
    fontSize: "large",
  })

  const member = css({
    display: "inline-block",
    fontSize: "18px",
    margin: "5px auto",
    padding: "5px",
    borderBottom: "2px solid",
  })

  return(
    <section class={container}>
      <div class={member}><span> التاريخ </span> : <span>{ date() }</span></div>
      <div class={member}><span> الوردية </span> : <span>{order()}</span></div>
    </section>
  )
}
