import { ReactNode, createContext, useCallback, useState } from "react";

import {
  InjectedAccountWithMeta,
  InjectedExtension,
} from "@polkadot/extension-inject/types";
import {
  web3Accounts,
  web3Enable,
  web3FromSource,
} from "@polkadot/extension-dapp";

type SubstrateProviderProps = {
  children: ReactNode;
};

type SubstrateContextType = {
  // accounts
  accounts: InjectedAccountWithMeta[];
  currentAccount: InjectedAccountWithMeta | null;
  setCurrentAccount: React.Dispatch<
    React.SetStateAction<InjectedAccountWithMeta | null>
  >;

  // api
  getExtension: () => Promise<InjectedExtension | null>;

  // extension
  web3enable: () => Promise<void>;
  web3enabled: boolean;
};

export const SubstrateContext = createContext<SubstrateContextType | null>(
  null
);

export function SubstrateProvider(props: SubstrateProviderProps) {
  const { children } = props;

  const [web3enabled, setWeb3enabled] = useState(false);
  const [accounts, setAccounts] = useState<InjectedAccountWithMeta[]>([]);
  const [currentAccount, setCurrentAccount] =
    useState<InjectedAccountWithMeta | null>(null);

  // callback to activate Polkadot extension
  const web3enable = useCallback(async () => {
    const extensions = await web3Enable("ScoreChain");
    console.log(
      "🚀 ~ file: connectProvider.tsx ~ line 54 ~ web3enable ~ allAccounts",
      extensions
    );
    if (extensions.length === 0) {
      console.error("No Web3 extension found!");
      return;
    } else {
      setWeb3enabled(true);
      const allAccounts = await web3Accounts();
      console.log(
        "🚀 ~ file: connectProvider.tsx ~ line 54 ~ web3enable ~ allAccounts",
        allAccounts
      );

      setAccounts(allAccounts);
      // by default, select the first account if available
      // if (allAccounts[0]) {
      //   setCurrentAccount(allAccounts[0]);
      // }
    }
  }, []);

  const getExtension = useCallback(async () => {
    if (!web3enabled) {
      await web3enable();
    }

    if (!currentAccount) {
      return null;
    }

    const injector = await web3FromSource(currentAccount.meta.source);

    // return InjectedExtension
    return injector;
  }, [web3enabled, web3enable, currentAccount]);

  const initialContextValue: SubstrateContextType = {
    accounts,
    currentAccount,
    setCurrentAccount,
    getExtension,
    web3enable,
    web3enabled,
  };

  return (
    <SubstrateContext.Provider value={initialContextValue}>
      {children}
    </SubstrateContext.Provider>
  );
}

export default SubstrateProvider;
