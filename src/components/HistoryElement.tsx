type HistoryElementParameter = {
  lockStatus: boolean;
  filename: string;
  location: string;
};
const HistoryElement = (props: HistoryElementParameter) => {
  const toggleFile = () => {
    console.log(props.location + props.filename);
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
