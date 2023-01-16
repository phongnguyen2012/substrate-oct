import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { SubstrateProvider } from "./api/providers/connectProvider";
import { BrowserRouter } from "react-router-dom";
import Router from "./routes/Router";
import HomePage from "./feature/homepage";

function App() {
  return (
    <SubstrateProvider>
      <BrowserRouter>
        <Router />
      </BrowserRouter>
    </SubstrateProvider>
  );
}

export default App;
