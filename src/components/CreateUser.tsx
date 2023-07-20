import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";

type createUserPrompts = {
  usernameId: string;
  firstPasswordId: string;
  secondPasswordId: string;
};

export default function CreateUser(prompts: createUserPrompts) {
  const navigate = useNavigate();

  const submitPassword = async () => {
    const username = document.getElementById(
      prompts.usernameId
    ) as HTMLInputElement;
    const firstPassword = document.getElementById(
      prompts.firstPasswordId
    ) as HTMLInputElement;
    const secondPassword = document.getElementById(
      prompts.secondPasswordId
    ) as HTMLInputElement;

    if (username.value.trim() == "") {
      alert("You have to enter a username");
      return;
    } else if (username.value.trim().length > 255) {
      alert("Username is to long is to long");
      return;
    } else if (
      firstPassword.value.trim() == "" ||
      secondPassword.value.trim() == ""
    ) {
      alert("Please enter your password correctly");
      return;
    } else if (firstPassword.value.trim() !== secondPassword.value.trim()) {
      alert("Passwords don't match");
      return;
    } else if (
      firstPassword.value.trim().length < 4 ||
      secondPassword.value.trim().length < 4
    ) {
      alert("Password is to short");
      return;
    }

    let answer = await invoke("create_new_user", {
      username: username.value,
      password: firstPassword.value.trim(),
      overwrite: false,
    }).then(() => {
      navigate("/app");
    });
  };

  return (
    <button className="create-user-button" onClick={submitPassword}>
      Create user
    </button>
  );
}
