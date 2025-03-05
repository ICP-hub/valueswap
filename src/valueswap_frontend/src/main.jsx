import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';
import { Provider } from 'react-redux';
import store from './Store';
import { AuthProvider } from './components/utils/useAuthClient';
import {
  IdentityKitProvider,
  IdentityKitTheme,
} from "@nfid/identitykit/react";

import {
  IdentityKitAuthType,
  NFIDW,
  Plug,
  InternetIdentity,
} from "@nfid/identitykit";
import "@nfid/identitykit/react/styles.css";


import { PrimeReactProvider } from 'primereact/api'; 
import 'primereact/resources/primereact.css';
import 'primereact/resources/themes/lara-light-indigo/theme.css';;
import { ToastContainer, toast } from 'react-toastify';

import 'react-toastify/dist/ReactToastify.css';  

// Define signers and canister ID
const signers = [NFIDW, Plug];
const additionalCanisterIds = [
    process.env.CANISTER_ID_CKBTC,
    process.env.CANISTER_ID_CKETH
];

// Combine all canister IDs
const whitelist = [
  process.env.CANISTER_ID_VALUESWAP_BACKEND,
  ...additionalCanisterIds
];

// Remove duplicates, if any
const uniqueWhitelist = [...new Set(whitelist)];

const signerClientOptions = {
  targets: [  process.env.CANISTER_ID_VALUESWAP_BACKEND],
  maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000), // 1 week in nanoseconds
  idleOptions: {
    idleTimeout: 4 * 60 * 60 * 1000, // 4 hours in milliseconds
    disableIdle: false, // Enable logout on idle timeout
  },
  keyType: 'Ed25519', // Use Ed25519 key type for compatibility
};

ReactDOM.createRoot(document.getElementById('root')).render(
  <IdentityKitProvider
    onConnectSuccess={(res) => {
      console.log("logged in successfully", res);
    }}
    onDisconnect={(res) => {
      console.log("logged out successfully", res);
    }}
    signers={signers}
    theme={IdentityKitTheme.SYSTEM}
    authType={IdentityKitAuthType.DELEGATION}
    signerClientOptions={signerClientOptions}
  >
    <Provider store={store}>
      <PrimeReactProvider>
      <AuthProvider>
        <App />
      <ToastContainer hideProgressBar={true} stacked theme="dark"/>
      </AuthProvider>
    </PrimeReactProvider>
    </Provider>
  </IdentityKitProvider>,
);
