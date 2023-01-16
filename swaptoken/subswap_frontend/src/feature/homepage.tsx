import * as React from "react";
import "../App.css";
interface IHomePageProps { }
import { Link, Route } from "react-router-dom";
import Market from "./market";
import Oracle from "./oracle";
import Token from "./token";

const HomePage: React.FunctionComponent<IHomePageProps> = (props) => {
  return<>
      <h3>Home</h3>
      <h3><Link to ="/oracle">Oracle </Link></h3>
      <h3><Link to ="/token">Token </Link></h3>
      <h3><Link to ="/market">Market</Link></h3>
      <h3>Vault</h3>
  </>
};

export default HomePage;
