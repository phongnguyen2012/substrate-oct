// import { decodeAddress, signatureVerify } from '@polkadot/util-crypto';
// import { u8aToHex, stringToHex } from '@polkadot/util';
// import { web3FromSource } from '@polkadot/extension-dapp';
// import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { ApiPromise, WsProvider } from "@polkadot/api";


//Staging mode
// const WS_PROVIDER = "wss://smartcv.org/smartcv-node";

//Develop mode
const WS_PROVIDER = "ws://127.0.0.1:9944";
let api: ApiPromise;

export async function getApi() {
  const wsProvider = new WsProvider(WS_PROVIDER);

  if (!api) {
    api = await ApiPromise.create({ provider: wsProvider });
    console.log("Connect to blockchain");
  }

  return api;
}

// export function isValidSignature(signedMessage: string, signature: string, address: string) {
//   const publicKey = decodeAddress(address);
//   const hexPublicKey = u8aToHex(publicKey);

//   return signatureVerify(signedMessage, signature, hexPublicKey).isValid;
// }

// export async function signMessage(message: string, currentAccount: InjectedAccountWithMeta) {
//   const injector = await web3FromSource(currentAccount.meta.source);
//   const signRaw = injector?.signer?.signRaw;

//   try {
//     if (!!signRaw) {
//       const { signature } = await signRaw({
//         address: currentAccount.address,
//         data: stringToHex(message),
//         type: 'bytes',
//       });

//       const verification = isValidSignature(message, signature, currentAccount.address);
//       if (verification) {
//         return signature;
//       } else {
//         throw new Error('Error while verifying message signature!');
//       }
//     } else {
//       throw new Error('Error while signing message!');
//     }
//   } catch (error) {
//     console.error(error);
//     return null;
//   }
// }

// type EXtrinsicResult = {
//   isSuccess: boolean;
//   hash: string;
//   errorMessage?: string;
// };

// export async function getSignedApi(currentAccount: InjectedAccountWithMeta) {
//   const api = await getApi();
//   const injector = await web3FromSource(currentAccount.meta.source);
//   api.setSigner(injector.signer);

//   return api;
// }

// export const getBigNumberAmount = (amount: number, chainDecimals: number) => {
//   BigNumber.set({ DECIMAL_PLACES: chainDecimals });
//   const bnAmount = new BigNumber(amount).times(10 ** chainDecimals).toNumber();
//   return bnAmount;
// };

// export const getAmountFromBigNumber = (amount: string | number, chainDecimals: number) => {
//   BigNumber.set({ DECIMAL_PLACES: chainDecimals });
//   const bnAmount = new BigNumber(amount).div(10 ** chainDecimals).toNumber();
//   return bnAmount;
// };
