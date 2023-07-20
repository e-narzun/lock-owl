import { invoke } from "@tauri-apps/api";
import { open } from "@tauri-apps/api/dialog";

export async function toggle_file(
  history: HistoryElementInfo[],
  historySetter: React.Dispatch<React.SetStateAction<HistoryElementInfo[]>>,
  SetSelectedPath: React.Dispatch<React.SetStateAction<string>>
) {
  open().then(async (path) => {
    if (path == null || Array.isArray(path)) {
      return;
    }
    if (path.endsWith("lowl")) {
      console.log(`Decrypt: ${path}`);
      SetSelectedPath(path);
    } else {
      console.log(`Encrypt: ${path}`);
      const backend_answer = await invoke("tauri_encrypt_file", {
        filePath: path,
        overwrite: false,
      }).then((result) => {
        let realResult = result as HistoryElementInfo;
        historySetter([realResult, ...history]);
      });
    }
  });
}
