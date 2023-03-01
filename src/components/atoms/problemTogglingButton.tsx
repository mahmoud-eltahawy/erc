
export default function togglingButton({
    showButton
    ,showMore,
     doOnClick
      } : {
     showButton : () => boolean,
     showMore   : () => boolean,
     doOnClick  : Function
      }){
    return(
        showButton() ? <button class="LongListButton"
                         onClick={() => doOnClick()}
                  >{showMore() ? "شاهد اكثر" : "شاهد اقل"}</button> : <></>
    )
}
