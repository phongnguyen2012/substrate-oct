import * as React from "react";
import { getApi } from "../api/config/utils";
import { useSubstrate } from "../api/providers/connectContext";
import Stack from '@mui/material/Stack';
import Button from '@mui/material/Button';
import {web3FromAddress } from '@polkadot/extension-dapp';
import { useState } from 'react';

export interface IRegisterProps {}

export default function Register(props: IRegisterProps) {
  const { getExtension, accounts } = useSubstrate();

  const [apiBC, setApiBC] = React.useState<any>();
  const callApi = async () => {
    const api = await getApi();

    setApiBC(api);
  };
  


  React.useEffect(() => {
    callApi();
    getExtension();
  }, []);

  const [result1, setResult1] = useState();
  const [result2, setResult2] = useState();

  const registerOperator = async () => {
    console.log("Call api");
    console.log("Current account:{}", accounts);
    if (accounts !== null ) {
      console.log("current Account:", accounts);
      const injector = await web3FromAddress(accounts[0].address);
    const events = new Promise(async (resolve, reject) => {
      //ordered param
      await apiBC.tx.oracleModule
      // fixed value
      // dynamic value
        .registerOperator()
        .signAndSend(
          accounts[0].address,
          { signer: injector?.signer },
          ({ status, events, dispatchError }: any) => {
            if (dispatchError) {
              if (dispatchError.isModule) {
                // for module errors, we have the section indexed, lookup
                const decoded = apiBC.registry.findMetaError(dispatchError.asModule);
                const { docs, name, section } = decoded;
                const res = 'Error'.concat(':', section, '.', name);
                //console.log(`${section}.${name}: ${docs.join(' ')}`);
                resolve(res);
              } else {
                // Other, CannotLookup, BadOrigin, no extra info
                //console.log(dispatchError.toString());
                resolve(dispatchError.toString());
              }
            } else {
              events.forEach(({ event, phase }: any) => {
                const { data, method, section } = event;
                //console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
                if (section == 'oracleModule') {
                  const res = 'Success'.concat(':', section, '.', method);
                  resolve(res);
                }
              });
            }
          }
        );
    });
    window.alert(await events);
  }
  
  }

  const operators = async () =>{
    const res = await apiBC.query.oracleModule.operators(accounts[0].address);
    setResult2(res.toString());
  }


  return <div>
    <h1> Register a new Operator. </h1>
    <Button variant="outlined" size="medium"  onClick={registerOperator}>
      Set status Oracle
    </Button>
    <p>
    <Button variant="outlined" size="medium"  onClick={operators}>
      Status
    </Button>
    {' '} Result: {result2}
    </p>
  </div>;
}
