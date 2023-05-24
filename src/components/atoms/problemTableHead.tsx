import { css } from "solid-styled-components";

export default function TableHead() {
  const style = css({
    width: "9%",
    paddingLeft: "7px",
    paddingRight: "7px",
    borderRight: "dotted 1px",
    borderLeft: "dotted 1px",
    borderBottom: "dotted 1px",
  });

  return (
    <thead>
      <tr>
        <th class={style}>تعديل</th>
        <th class={style}>ملحوظة جانبية</th>
        <th class={style}>(24)التوقيت</th>
        <th class={style}>قطع الغيار</th>
        <th class={style}>المشاكل</th>
        <th class={style}>القائم باصلاح العطل</th>
        <th class={style}>الماكينة التي حدث عليها العطل</th>
      </tr>
    </thead>
  );
}
