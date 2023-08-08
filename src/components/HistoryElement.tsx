import { invoke } from "@tauri-apps/api";

type HistoryElementParameter = {
  lockStatus: boolean;
  filename: string;
  location: string;
  historyList: HistoryElementInfo[];
  historySetter: React.Dispatch<React.SetStateAction<HistoryElementInfo[]>>;
  setSelectedPath: React.Dispatch<React.SetStateAction<string>>;
};
const HistoryElement = (props: HistoryElementParameter) => {
  const toggleFile = async () => {
    console.log(props.location + props.filename);
    let path = props.location;
    if (path == null || Array.isArray(path)) {
      return;
    }
    if (path.endsWith("lowl")) {
      console.log(`Decrypt: ${path}`);
      props.setSelectedPath(path);
    } else {
      console.log(`Encrypt: ${path}`);
      const backend_answer = await invoke("tauri_encrypt_file", {
        filePath: path,
        overwrite: false,
      }).then((result) => {
        let realResult = result as HistoryElementInfo;
        props.historySetter([realResult, ...props.historyList]);
      });
    }
  };

  return (
    <div className="history-element-wrapper">
      <button
        className={"status" + (props.lockStatus ? " locked" : " unlocked")}
        onClick={toggleFile}
      >
        <img src={props.lockStatus ? "/lock.svg" : "/lock_open.svg"} />
      </button>
      <div className="description">
        <h3>{props.filename}</h3>
        <p>{props.location}</p>
      </div>
    </div>
  );
};
export default HistoryElement;
