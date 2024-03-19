import { convertFileSrc } from "@tauri-apps/api/core";

import { EntryDetails } from "../services/api/entry";

interface Props {
  entry: EntryDetails | null;
}

export function OmniPlayer(props: Props) {
  return (
    <>
      {props.entry ? (
        <img src={convertFileSrc(props.entry.meta.path)} />
      ) : (
        <></>
      )}
    </>
  );
}
