import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import App from "./App";
import CustomRouter from "./CustomRouter";
import "./style/color.scss";
import "./style/root.scss";
import "./style/components.scss";

document.addEventListener("contextmenu", (event) => event.preventDefault());

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <CustomRouter />
    </BrowserRouter>
  </React.StrictMode>
);
