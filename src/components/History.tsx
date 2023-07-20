import "../style/history.scss";
import HistoryElement from "./HistoryElement";

type historyProps = {
  historyList: HistoryElementInfo[];
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
          ></HistoryElement>
        ))}
      </div>
    </div>
  );
}
