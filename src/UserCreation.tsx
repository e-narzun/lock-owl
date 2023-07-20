import "./style/userCreation.scss";
import TextInput from "./components/TextInput";
import CreateUser from "./components/CreateUser";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api";

export default function UserCreation() {
  const checkPasswords = () => {
    let first_password = document.getElementById(
      "password-first"
    ) as HTMLInputElement;
    let second_password = document.getElementById(
      "password-second"
    ) as HTMLInputElement;
    if (
      second_password.value !== "" &&
      second_password.value !== first_password.value
    ) {
      second_password.style.outline = "red solid 1px";
    } else if (second_password.value !== "") {
      second_password.style.outline = "transparent";
    } else {
      second_password.style.outline = "green solid 1px";
    }
  };

  const navigate = useNavigate();

  invoke("check_private_key").then(() => {
    navigate("/app");
  });

  return (
    <div className="user-creation">
      <h1 className="header">Create new profile</h1>
      <div className="input-files">
        <TextInput id="username" title="Username" password={false}></TextInput>
        <TextInput
          id="password-first"
          title="Password"
          password={true}
        ></TextInput>
        <TextInput
          id="password-second"
          title="Confirm password"
          password={true}
        ></TextInput>
      </div>
      <CreateUser
        usernameId="username"
        firstPasswordId="password-first"
        secondPasswordId="password-second"
      ></CreateUser>
    </div>
  );
}
