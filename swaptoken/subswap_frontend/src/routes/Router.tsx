import { Route, Routes } from "react-router-dom";
import HomePage from "../feature/homepage";
import Oracle from "../feature/oracle";
import Token from "../feature/token";
import Market from "../feature/market";


import Test from "../feature/test";

function Router() {
  return (
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/oracle" element={<Oracle/>} />
      <Route path="/token" element={<Token/>} />
      <Route path="/market" element={<Market/>} />
      <Route path="/test" element={<Test/>} />
    </Routes>
  );
}

export default Router;
