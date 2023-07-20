import { Route, Routes } from "react-router-dom";
import App from "./App";
import UserCreation from "./UserCreation";

export default function CustomRouter() {
  return (
    <Routes>
      <Route path="/" element={<UserCreation />}></Route>
      <Route path="/app" element={<App />}></Route>
    </Routes>
  );
}
