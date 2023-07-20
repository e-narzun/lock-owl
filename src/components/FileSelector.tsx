import { toggle_file } from "../logic/crypto";

type FileSelectorProps = {
  historyList: HistoryElementInfo[];
  historySetter: React.Dispatch<React.SetStateAction<HistoryElementInfo[]>>;
  setSelectedPath: React.Dispatch<React.SetStateAction<string>>;
};

export default function FileSelector(props: FileSelectorProps) {
  return (
    <>
      <button
        className="button-file-path"
        onClick={() =>
          toggle_file(
            props.historyList,
            props.historySetter,
            props.setSelectedPath
          )
        }
      >
        <img src="/plus-circle.svg" />
        <span>Add new file</span>
      </button>
    </>
  );
}
