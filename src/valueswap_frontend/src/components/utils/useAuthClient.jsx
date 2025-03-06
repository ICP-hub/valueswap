
// // plug integration

// import { AuthClient } from "@dfinity/auth-client";
// import React, { createContext, useCallback, useContext, useEffect, useState } from "react";
// import { HttpAgent, Actor } from "@dfinity/agent";
// import { Principal } from "@dfinity/principal";
// import { createActor as createActorBackend, idlFactory } from '../../../../declarations/valueswap_backend/index';
// import { PlugLogin, StoicLogin, NFIDLogin, IdentityLogin } from "ic-auth";
// import { createActor as ledgerActor, idlFactory as TokenIdl } from "../../../../declarations/ckbtc/index";
// import {  idlFactory as ckETHIdlFactory } from "../../../../declarations/cketh/index";
// import { DummyDataTokens } from '../../TextData';
// // import { PlugMobileProvider } from '@funded-labs/plug-mobile-sdk'
// import { useBalance, useIdentity, useDelegationType, useIsInitializing, useAgent} from '@nfid/identitykit/react';
// import {useAuth} from '@nfid/identitykit/react';


// const AuthContext = createContext();

// export const useAuthClient = () => {
//   const [isAuthenticated, setIsAuthenticated] = useState(false);
//   const [authClient, setAuthClient] = useState(null);
//   const [principal, setPrincipal] = useState(null); // Holds Principal object
//   const [backendActor, setBackendActor] = useState(null);
//   const [provider, setProvider] = useState(null); // Keep track of the provider
//   // const isMobile = PlugMobileProvider.isMobileBrowser()

//   const { connect, disconnect, isConnecting, user } = useAuth();
//   const { balance, fetchBalance } = useBalance();
//   const identity = useIdentity();
//   // const accounts = useAccounts();
//   const delegationType = useDelegationType();
//   const isInitializing = useIsInitializing();
//   const agent = useAgent();

//   const LOCAL_HOST = "http://127.0.0.1:4943";
//   const MAINNET_HOST = "https://ic0.app";
//   const HOST = process.env.DFX_NETWORK === "ic" ? MAINNET_HOST : LOCAL_HOST;
  
//   useEffect(() => {

//     // AuthClient.create().then((client) => {
//     //   setAuthClient(client);
//     // });

//     const initActor = async () => {
//       try {
//         if (user && identity && agent) {
//           // Fetch root key for local development
//           if (process.env.DFX_NETWORK !== "ic") {
//             await agent.fetchRootKey();
//           }

//           // Create actor
//           const actor = createActor(canisterID, { agent });
//           setBackendActor(actor);
//         }
//       } catch (error) {
//         console.error("Error initializing actor:", error.message);
//       }
//     };
//     debugReadState(agent, canisterID);
//     testSigning();
//     initActor();
//   }, [user, identity, agent]);

//   // useEffect(() => {
//   //   if (authClient) {
//   //     updateClient(authClient);
//   //   }
//   // }, [authClient]);

//   const login = async () => {
//     // try {
//     //   setProvider(selectedProvider); // Set the provider
    
//     //   const additionalCanisterIds = [
//     //     process.env.CANISTER_ID_CKBTC,
//     //     process.env.CANISTER_ID_CKETH
//     //   ];

//     //   // Combine all canister IDs
//     //   const whitelist = [
//     //     process.env.CANISTER_ID_VALUESWAP_BACKEND,
//     //     ...additionalCanisterIds
//     //   ];

//     //   // Remove duplicates, if any
//     //   const uniqueWhitelist = [...new Set(whitelist)];

//     //   if (selectedProvider === "plug") {
//     //     // Plug login
//     //     // if (isMobile) {
//     //     //   const provider = new PlugMobileProvider({
//     //     //     debug: true, // If you want to see debug logs in console
//     //     //     walletConnectProjectId: '6e2de4a3633b8ad436730aea43901ef3', // Project ID from WalletConnect console
//     //     //     window: window,
//     //     //   })
//     //     //   // setProvider(provider)
//     //     //   provider.initialize().catch(console.log)
//     //     //   setProvider(provider)

//     //     //   if (!provider.isPaired()) {
//     //     //     provider.pair().catch(console.log)
//     //     //   }

         
//     //     //     const agent = await provider.createAgent({
//     //     //       host: 'https://icp0.io',
//     //     //       targets: [whitelist], // List of canister you are planning to call
//     //     //     })
        
//     //     // const backendActor = createActorBackend(process.env.CANISTER_ID_VALUESWAP_BACKEND, {agent:agent});
//     //     // const principal = agent.getPrincipal()
//     //     // setBackendActor(backendActor);
//     //     // setPrincipal(principal);
//     //     // setIsAuthenticated(true);
//     //     // }
//     //     // Collect all canister IDs you need to whitelist
       
//     //     // Ensure all canister IDs are valid
//     //     if (uniqueWhitelist.includes(undefined) || uniqueWhitelist.includes('')) {
//     //       console.error("One or more canister IDs are undefined or empty. Please check your environment variables.");
//     //       console.error("Whitelist:", uniqueWhitelist);
//     //       return;
//     //     }
  
//     //     // Check if Plug is installed
//     //     if (!window.ic || !window.ic.plug) {
//     //       console.error("Plug wallet is not installed.");
//     //       return;
//     //     }
  
//     //     // Request connection with the necessary whitelist
//     //     const result = await window.ic.plug.requestConnect({
//     //       whitelist: uniqueWhitelist,
//     //       host: process.env.DFX_NETWORK === "local" ? 'http://localhost:3000' : 'https://mainnet.dfinity.network', // or your local network
          
//     //     });
  
//     //     if (!result) {
//     //       console.error("User denied the connection.");
//     //       return;
//     //     }
//     //     console.log("result o f infinity",result)
//     //     // After connecting, the agent should be initialized
//     //     if (!window.ic.plug.agent) {
//     //       console.error("Plug agent is not initialized.");
//     //       return;
//     //     }
  
//     //     // Get the principal via the agent
//     //     const principal = await window.ic.plug.agent.getPrincipal();
//     //     console.log("Principal:", principal.toString());
//     //     setPrincipal(principal); // Store the Principal object
//     //     setIsAuthenticated(true);
  
//     //     // Create the backend actor using Plug's createActor method
//     //     const backendActor = await window.ic.plug.createActor({
//     //       canisterId: process.env.CANISTER_ID_VALUESWAP_BACKEND,
//     //       interfaceFactory: idlFactory, // Ensure idlFactory is imported correctly
//     //     });
//     //     setBackendActor(backendActor);
  
//     //     // Plug does not expose identity directly
//     //     setIdentity(null);
  
//     //     console.log("Plug login successful.");
//     //   }else if( selectedProvider === "bitfinityWallet"){ 
//     //     // Check if bitfinityWallet is installed
//     //     if (!window.ic || !window.ic.infinityWallet) {
//     //       console.error("bitfinityWallet wallet is not installed.");
//     //       return;
//     //     }
//     //     const result = await window.ic.infinityWallet.requestConnect({
//     //       whitelist: uniqueWhitelist,
//     //       host: process.env.DFX_NETWORK === "local" ? 'http://localhost:3000' : 'https://mainnet.dfinity.network',
//     //        // or your local network
          
//     //     });
//     //     console.log("result o f infinity",result)
//     //     if (!result) {
//     //       console.error("User denied the connection.");
//     //       return;
//     //     }
  
//     //     // After connecting, the agent should be initialized
//     //     // if (!window.ic.infinityWallet.agent) {
//     //     //   console.error("bitfinityWallet agent is not initialized.");
//     //     //   return;
//     //     // }
  
//     //     // Get the principal via the agent
//     //     const principal = await window.ic.infinityWallet.getPrincipal();
//     //     console.log("Principal:", principal.toString());
//     //     setPrincipal(principal); // Store the Principal object
//     //     setIsAuthenticated(true);
  
//     //     // Create the backend actor using Plug's createActor method
//     //     const backendActor = await window.ic.infinityWallet.createActor({
//     //       canisterId: process.env.CANISTER_ID_VALUESWAP_BACKEND,
//     //       interfaceFactory: idlFactory,
//     //       host: process.env.DFX_NETWORK === 'local' ? 'http://localhost:3000' : 'https://mainnet.dfinity.network', // Ensure idlFactory is imported correctly
//     //     });
//     //     setBackendActor(backendActor);
  
//     //     // Plug does not expose identity directly
//     //     setIdentity(null);
  
//     //     console.log("bitfinityWallet login successful.");

//     //   }else if (selectedProvider === "stoic") {
//     //     // Stoic login
//     //     const userObject = await StoicLogin();
//     //     const identity =   userObject.agent._identity; 
//     //     // console.log("identity", StoicLogin())// StoicLogin returns identity
//     //     const principal = await userObject.principal;
//     //     console.log("principal stoic", principal, userObject)
//     //     setPrincipal(principal); // Store the Principal object
//     //     setIdentity(identity);
//     //     setIsAuthenticated(true);
  
//     //     const backendActor = createActorBackend(
//     //       process.env.CANISTER_ID_VALUESWAP_BACKEND,
//     //       { agentOptions: { identity } }
//     //     );
//     //     setBackendActor(backendActor);
  
//     //     console.log("Stoic login successful.");
//     //   } else if (selectedProvider === "nfid" || selectedProvider === "ii") {
//     //     // NFID and II login
//     //     const identityProvider =
//     //       selectedProvider === "nfid"
//     //         ? 'https://nfid.one/authenticate/'
//     //         : 'https://identity.ic0.app/#authorize';
  
//     //     await authClient.login({
//     //       identityProvider,
//     //       onSuccess: async () => {
//     //         await updateClient(authClient);
//     //         console.log(`${selectedProvider} login successful.`);
//     //       },
//     //       onError: (error) => {
//     //         console.error(`${selectedProvider} login error:`, error);
//     //       },
//     //     });
//     //   }
//     // } catch (error) {
//     //   console.error("Login error:", error);
//     // }
//     try {
//       await connect();
//       const principal = identity.getPrincipal();
//       setPrincipal(principal);
//     } catch (error) {
//       console.error("Login Error:", error);
//     }
//   };
  

//   const logout = async () => {
//     try {
//       await disconnect();
//       setBackendActor(null);
//     } catch (error) {
//       console.error("Logout Error:", error);
//     }
//   };

//   const updateClient = async (client) => {
//     const isAuthenticated = await client.isAuthenticated();
//     setIsAuthenticated(isAuthenticated);
//     const identity = client.getIdentity();
//     setIdentity(identity);

//     const principal = identity.getPrincipal();
//     setPrincipal(principal); // Store the Principal object

//     const backendActor = createActorBackend(
//       process.env.CANISTER_ID_VALUESWAP_BACKEND,
//       { agentOptions: { identity } }
//     );
//     setBackendActor(backendActor);
//   };

//   const createTokenActor = async (canisterID) => {
//       const tokenActor = ledgerActor(canisterID, { agentOptions: { identity } });
//       return tokenActor;
//   };

//   const testSigning = async () => {
//     try {
//       const encoder = new TextEncoder();
//       const payload = encoder.encode("Test payload");
//       const signature = await identity.sign(payload);
//       console.log("Test Signature:", signature);
//     } catch (error) {
//       console.error("Error in signing:", error.message);
//     }
//   };



//   const getBalance = useCallback(async (canisterId) => {
  
//     if (provider === "plug") {
//         try { 
//             const tokenActor = await window.ic.plug.createActor({
//                 canisterId: canisterId,
//                 interfaceFactory: TokenIdl, // Ensure TokenIdl matches the canister's IDL
//               });
    
//             console.log("Token actor:", tokenActor);
    
//             // const ownerPrincipal = typeof principal === 'string' ? Principal.fromText(principal) : principal;
    
//             const balance = await tokenActor.icrc1_balance_of({ owner: principal, subaccount: [] });
//             console.log("Token balance:", balance);
//              setBalance(balance);
//              return balance;
//           } catch (error) {
//             console.error("Error fetching balance from Plug:", error);
//             return null;
//           }
//     } else {
//       // Handle other providers
//       try {
//         // Assuming you have a function to create an actor for other providers
//         const actor = await createTokenActor(canisterId);
  
//         const ownerPrincipal = typeof principal === 'string' ? Principal.fromText(principal) : principal;
     
//         const balance = await actor?.icrc1_balance_of({ owner: ownerPrincipal,  subaccount: []});
//         console.log("Balance:", balance.toString());
//         return balance;
//       } catch (error) {
//         console.error("Error fetching balance from other provider:", error);
//         return null;
//       }
//     }
//   }, [provider, principal]);

//   const fetchMetadata = async (CanisterId) => {
//     try {
//       const ledgerActor = await createTokenActor(CanisterId);
//       const result = await ledgerActor?.icrc1_metadata();
//       console.log("Fetched metadata:", result);
  
//       // Extract decimals and symbol from the metadata
//       const decimalsEntry = result.find(([key]) => key === "icrc1:decimals");
//       const symbolEntry = result.find(([key]) => key === "icrc1:symbol");
  
//       const decimals = decimalsEntry ? Number(decimalsEntry[1]?.Nat) : null; // Convert BigInt to Number
//       const symbol = symbolEntry ? symbolEntry[1]?.Text : null;
//       console.log("meta", decimals, symbol)
//       return {
//         decimals,
//         symbol,
//       };
//     } catch (error) {
//       console.error("Error fetching metadata:", error);
//       return null; // Return null in case of an error
//     }
//   };

//   const debugReadState = async (agent, canisterId) => {
//     try {
//       const paths = [[new TextEncoder().encode("time")]];
//       const state = await agent.readState(canisterId, { paths });
//       console.log("Read State Response:", state);
//     } catch (error) {
//       console.error("Error in readState:", error.message);
//     }
//   };
  
  
//   return {
//     isAuthenticated : !!user,
//     login,
//     logout,
//     authClient,
//     identity,
//     principal,
//     backendActor,
//     getBalance : fetchBalance,
//     createTokenActor,
//     balance,
//     fetchMetadata
//   };
// };

// export const AuthProvider = ({ children }) => {
//   const auth = useAuthClient();
//   return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
// };
// export const useAuths = () => useContext(AuthContext);


import React, { createContext, useContext, useEffect, useState } from "react";
// import { createActor } from "../../../declarations/master";
import { useBalance } from "@nfid/identitykit/react";
import { useIdentity } from "@nfid/identitykit/react";
import { useDelegationType } from "@nfid/identitykit/react";
import { useIsInitializing } from "@nfid/identitykit/react";
import { useAgent } from "@nfid/identitykit/react";
import { useAuth } from "@nfid/identitykit/react"

// import showNotification from "../reusable_components/showNotification";
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory as ledgerIDL } from "./ledger.did.js";
import { createActor as ledgerActor, idlFactory as TokenIdl } from "../../../../declarations/ckbtc/index";
// import {  idlFactory as ckETHIdlFactory } from "../../../../declarations/cketh/index";
// import { useNavigate } from "react-router-dom";

const AuthContext = createContext();
const canisterID = process.env.CANISTER_ID_VALUESWAP_BACKEND;
export const useAuthClient = () => {
  const { connect, disconnect, isConnecting, user } = useAuth();
  const { balance, fetchBalance } = useBalance();
  const identity = useIdentity();
  const delegationType = useDelegationType();
  const isInitializing = useIsInitializing();
  const [backendActor, setBackendActor] = useState(null);
  const [agent, setnewagent] = useState(null);
  const LOCAL_HOST = "http://127.0.0.1:4943";
  const MAINNET_HOST = "https://icp0.io";
  const HOST = process.env.DFX_NETWORK === "ic" ? MAINNET_HOST : LOCAL_HOST;

  useEffect(() => {
    if (user && identity && HOST) {
      const initializeAgent = async () => {
        const newAgent = new HttpAgent({ identity, host: HOST });
        if (process.env.DFX_NETWORK !== "ic") {
          await newAgent.fetchRootKey();
        }
        setnewagent(newAgent);
      };
      initializeAgent();
    }
  }, [user, identity, HOST]);

  const handleLogin = async () => {
    try {
      await connect();
      const principal = identity.getPrincipal().toText();
      // showNotification("success", "Wallet Connected", principal);
    } catch (error) {
      console.error("Login Error:", error);
    }
  };

  const handleLogout = async () => {
    try {
      console.log("Initiating logout...");
      await disconnect();
      console.log("Disconnected successfully");
      // showNotification("success", "Logout Successful");
      setBackendActor(null);
      console.log("Navigating to '/'...");
      // navigate("/", { replace: true });
      console.log("Navigation triggered");
    } catch (error) {
      console.error("Logout Error:", error.message, error.stack);
      // showNotification("error", "Logout Failed");
    }
  };

  const createCustomActor = async (canisterId) => {
    try {
      if (!canisterId) {
        throw new Error("Canister ID is required.");
      }
      const agent = new HttpAgent({ identity, host: HOST });

      // Only fetch root key in local development
      if (process.env.DFX_NETWORK !== "ic") {
        await agent.fetchRootKey();
      }

      const actor = Actor.createActor(ledgerIDL, { agent, canisterId });
      if (!actor) {
        throw new Error(
          "Actor creation failed. Check the IDL and canister ID."
        );
      }
      return actor;
    } catch (err) {
      console.error("Error creating custom actor:", err.message);
      throw err;
    }
  };
  const signerId = localStorage.getItem("signerId");

  return {
    isInitializing,
    isAuthenticated: !!user,
    isConnecting,
    identity,
    backendActor,
    createCustomActor,
    delegationType,
    login: handleLogin,
    principal: user?.principal?.toText() || null,
    logout: handleLogout,
    agent,
    fetchBalance,
    actor: ledgerActor(canisterID, {
      agentOptions: { identity, verifyQuerySignatures: false },
    }),
    signerId
  };
};

export const AuthProvider = ({ children }) => {
  const auth = useAuthClient();
  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
};

export const useAuths = () => useContext(AuthContext);
