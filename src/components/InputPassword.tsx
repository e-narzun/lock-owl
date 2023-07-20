import TextInput from "./TextInput";
import { invoke } from "@tauri-apps/api";
type InputPasswordProps = {
  selectedPath: string;
  setSelectedPath: React.Dispatch<React.SetStateAction<string>>;
  historyList: HistoryElementInfo[];
  setHistoryList: React.Dispatch<React.SetStateAction<HistoryElementInfo[]>>;
};

export default function InputPassword(props: InputPasswordProps) {
  const onSubmit = async () => {
    const passwordField = document.getElementById(
      "password-input-filed"
    ) as HTMLInputElement;

    const backend_answer = await invoke("tauri_decrypt_file", {
      filePath: props.selectedPath,
      overwrite: false,
      password: passwordField.value.trim(),
    })
      .then((result) => {
        let realResult = result as HistoryElementInfo;
        console.log(realResult);
        props.setHistoryList([realResult, ...props.historyList]);
      })
      .catch(() => {
        alert("Unable to decrypt");
      });
    props.setSelectedPath("");
  };

  const onCancel = () => {
    props.setSelectedPath("");
  };

  return (
    <div className="password-input-background">
      <div className="password-input-wrapper">
        <TextInput
          title=""
          id="password-input-filed"
          password={true}
        ></TextInput>
        <div></div>
        <button className="submit" onClick={onSubmit}>
          Encrypt file
        </button>
        <button className="cancel" onClick={onCancel}>
          Cancel
        </button>
      </div>
    </div>
  );
}
