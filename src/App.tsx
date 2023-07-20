import FileSelector from "./components/FileSelector";
import History from "./components/History";
import InputPassword from "./components/InputPassword";
import "./style/app.scss";
import { useState } from "react";

declare global {
  type HistoryElementInfo = {
    filename: string;
    path: string;
    status: boolean;
  };
}
export default function App() {
  let [historyList, setHistoryList] = useState<HistoryElementInfo[]>([]);
  const [password, setPassword] = useState<string>("");
  const [selectedPath, setSelectedPath] = useState<string>("");

  return (
    <div className="app">
      <History historyList={historyList} />
      <FileSelector
        historyList={historyList}
        historySetter={setHistoryList}
        setSelectedPath={setSelectedPath}
      />
      {selectedPath !== "" && (
        <InputPassword
          selectedPath={selectedPath}
          setSelectedPath={setSelectedPath}
          historyList={historyList}
          setHistoryList={setHistoryList}
        ></InputPassword>
      )}
    </div>
  );
}
