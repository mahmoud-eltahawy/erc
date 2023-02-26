import { invoke } from '@tauri-apps/api/tauri'
import { createResource } from 'solid-js';

const fetcher = async () => {
  const [order,date] = await invoke('current_shift') as [string,[String, String, String]]
  return [date.join(" / "),order]
}
export default function ShiftIdentity(){
  const [shift,{refetch}] = createResource(fetcher)

  const date  = () => (shift() || []).at(0)
  const order = () => (shift() || []).at(1)

  setInterval(() => refetch(),60000)

  return(
    <section class={"shiftIdContainer"}>
      <div class={"shiftIdMember"}><span> التاريخ </span> : <span>{ date() }</span></div>
      <div class={"shiftIdMember"}><span> الوردية </span> : <span>{order()}</span></div>
    </section>
  )
}
