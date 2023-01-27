import { invoke } from '@tauri-apps/api/tauri'
import { useEffect, useState } from 'react';
import "./App.css";

export default function ShiftIdentity(){
    const [date, setdate] = useState("");
    const [order,setorder] = useState("");

    useEffect(() => {
        invoke('current_shift').then(result => {
            let [date,orde] = result as [string, [String, String, String]]
            const order = orde.join(" / ")
            setdate(date)
            setorder(order)
        }).catch(err => {
            invoke('update_current_shift').then(() => {
                invoke('current_shift').then(result => {
                    let [date,orde] = result as [string, [String, String, String]]
                    const order = orde.join(" / ")
                    setdate(date)
                    setorder(order)
                }).catch(err => {
                    alert(err)
                })
            })
        })
    })

    return(
        <section className={"shiftIdContainer"}>
            <div className={"shiftIdMember"}><span> التاريخ </span> : <span>{ date }</span></div>
            <div className={"shiftIdMember"}><span> الوردية </span> : <span>{order}</span></div>
        </section>
    )
}
