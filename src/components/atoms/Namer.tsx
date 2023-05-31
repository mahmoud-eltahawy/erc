import { invoke } from "@tauri-apps/api";
import { createResource } from "solid-js";

export default function Namer(props: {
  id: () => string;
  command: string;
}) {
  const fetcher = async () => {
    return (await invoke(props.command, { id: props.id() })
      .catch((err) => console.log(err))) as string;
  };

  const [name] = createResource(fetcher);

  return <p>{name()}</p>;
}
