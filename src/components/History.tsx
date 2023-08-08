import "../style/history.scss";
import HistoryElement from "./HistoryElement";

type historyProps = {
  historyList: HistoryElementInfo[];
  historySetter: React.Dispatch<React.SetStateAction<HistoryElementInfo[]>>;
  setSelectedPath: React.Dispatch<React.SetStateAction<string>>;
};

export default function History(props: historyProps) {
  console.log(props.historyList);

  return (
    <div className="history">
      <h1 className="header">History</h1>
      <div className="history-list">
        {props.historyList.map((element) => (
          <HistoryElement
            lockStatus={element.status}
            filename={element.filename}
            location={element.path}
            historyList={props.historyList}
            historySetter={props.historySetter}
            setSelectedPath={props.setSelectedPath}
          ></HistoryElement>
        ))}
      </div>
    </div>
  );
}
