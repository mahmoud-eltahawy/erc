import { invoke } from '@tauri-apps/api/tauri'
import { createEffect, createSignal } from 'solid-js';

export default function ShiftIdentity(){
  const [date, setdate] = createSignal("");
  const [order,setorder] = createSignal("");

  createEffect(() => {
    invoke('current_shift').then(result => {
      let [order,dat] = result as [string, [String, String, String]]
      const date = dat.join(" / ")
      setdate(date)
      setorder(order)
    }).catch(err => {
      console.log(err)
    })
  })

  return(
    <section class={"shiftIdContainer"}>
      <div class={"shiftIdMember"}><span> التاريخ </span> : <span>{ date }</span></div>
      <div class={"shiftIdMember"}><span> الوردية </span> : <span>{order}</span></div>
    </section>
  )
}
