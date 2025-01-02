// import { AuthClient } from "@dfinity/auth-client";
// import React, { createContext, useContext, useEffect, useState } from "react";
// import {
//     createActor as createActorBackend, 
// } from '../../../../declarations/valueswap_backend/index';
// // import { Actor, HttpAgent } from "@dfinity/agent";
// import { createActor as ledgerActor, idlFactory as TokenIdl} from "../../../../declarations/ckbtc/index"


// const AuthContext = createContext();

// const defaultOptions = {
//     /**
//      *  @type {import("@dfinity/auth-client").AuthClientCreateOptions}
//      */
//     createOptions: {
//         // idleOptions: {
//         //   // Set to true if you do not want idle functionality
//         //   disableIdle: true,
//         // },
//         idleOptions: {
//             idleTimeout: 1000 * 60 * 30, // set to 30 minutes
//             disableDefaultIdleCallback: true, // disable the default reload behavior
//         },
//     },
//     /**
//      * @type {import("@dfinity/auth-client").AuthClientLoginOptions}
//      */
//     loginOptionsIcp: {
//         identityProvider:
//             process.env.DFX_NETWORK === "ic"
//                 ? "https://identity.ic0.app/#authorize"
//                 : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/`,
//     },
//     loginOptionsnfid: {
//         identityProvider:
//             process.env.DFX_NETWORK === "ic"
//                 ? `https://nfid.one/authenticate/?applicationName=my-ic-app#authorize`
//                 : `https://nfid.one/authenticate/?applicationName=my-ic-app#authorize`
//     },
// };

// /**
//  *
//  * @param options - Options for the AuthClient
//  * @param {AuthClientCreateOptions} options.createOptions - Options for the AuthClient.create() method
//  * @param {AuthClientLoginOptions} options.loginOptions - Options for the AuthClient.login() method
//  * @returns
//  */
// export const useAuthClient = (options = defaultOptions) => {
//     const [isAuthenticated, setIsAuthenticated] = useState(false);
//     const [authClient, setAuthClient] = useState(null);
//     const [identity, setIdentity] = useState(null);
//     const [principal, setPrincipal] = useState(null);
//     const [balance, setBalance] = useState(null);
//    const [backendActor, setBackendActor] = useState(null)
//     useEffect(() => {
//         // Initialize AuthClient
//         AuthClient.create(options.createOptions).then((client) => {
//             setAuthClient(client);
//         });
//     }, []);
//  const backendCanisterId = process.env.CANISTER_ID_VALUESWAP_BACKEND;
//  const login = (val) => {
//     return new Promise(async (resolve, reject) => {
//         try {
//             if (authClient.isAuthenticated() && !authClient.getIdentity().getPrincipal().isAnonymous()) {
//                 const backendActor = await createActorBackend(backendCanisterId, { agentOptions: { identity } });
//                 setBackendActor(backendActor); // Set the backend actor with authenticated identity
//                 updateClient(authClient);
//                 resolve(authClient);
//             } else {
//                 let opt = val === "Icp" ? "loginOptionsIcp" : "loginOptionsnfid";
//                 authClient.login({
//                     ...options[opt],
//                     onError: (error) => reject(error),
//                     onSuccess: async () => {
//                         await updateClient(authClient);
//                         const backendActor = await createActorBackend(backendCanisterId, { agentOptions: { identity: authClient.getIdentity() } });
//                         setBackendActor(backendActor); // Set backend actor after successful login
//                         resolve(authClient);
//                     },
//                 });
//             }
//         } catch (error) {
//             reject(error);
//         }
//     });
// };

//     // const createTokenActor = (canisterId) => {
//     //     if(!canisterId){
//     //         throw new Error("Canister ID is undefined");
//     //     }
//     //     const agent = new HttpAgent();
//     //     let tokenActor = createActor(TokenIdl, {
//     //         agent,
//     //         canisterId: canisterId,
//     //     });
//     //     return tokenActor;
//     // };

//     const reloadLogin = () => {
//         return new Promise(async (resolve, reject) => {
//             try {
//                 if (authClient.isAuthenticated() && ((await authClient.getIdentity().getPrincipal().isAnonymous()) === false)) {
//                     updateClient(authClient);
//                     resolve(AuthClient);
//                 }
//             } catch (error) {
//                 reject(error);
//             }
//         })
//     };


// async function updateClient(client) {
//     const isAuthenticated = await client.isAuthenticated();
//     setIsAuthenticated(isAuthenticated);
//     const identity = client.getIdentity();
//     setIdentity(identity);
//     const principal = identity.getPrincipal();
//     setPrincipal(principal);
//     setAuthClient(client);

//     if (isAuthenticated && !principal.isAnonymous()) {
//         const backendActor = await createActorBackend(backendCanisterId, { agentOptions: { identity } });
//         setBackendActor(backendActor); // Create backend actor with the authenticated identity
//     }
// }


  

//     async function logout() {
//         await authClient?.logout();
//         await updateClient(authClient);
//         setIsAuthenticated(false);
//     }


//     const createTokenActor = (canisterID) => {
//         let tokenActor = ledgerActor(canisterID, { agentOptions: { identity: identity } })
//         return tokenActor;
        
//             }
        

//     const canisterId = process.env.CANISTER_ID_CKETH

//     // const actor = createActorBackend(canisterId, { agentOptions: { identity } });

// const getBalance = async (principal, canisterId) =>{
//     const actor = await createTokenActor(canisterId)
//     const balance = await actor.icrc1_balance_of({ owner: principal, subaccount: [] })
//     setBalance(balance)
//     return balance;
//    }


//     return {
//        login,
//         logout,
//         principal,
//         isAuthenticated,
//         setPrincipal,
//         createTokenActor,
//         identity,
//         getBalance,
//         balance,
//         backendActor
//     };
// };

// /**
//  * @type {React.FC}
//  */
// export const AuthProvider = ({ children }) => {
//     const auth = useAuthClient();
//     return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
// };

// export const useAuth = () => useContext(AuthContext);







// plug integration

import { AuthClient } from "@dfinity/auth-client";
import React, { createContext, useCallback, useContext, useEffect, useState } from "react";
import { HttpAgent } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { createActor as createActorBackend, idlFactory } from '../../../../declarations/valueswap_backend/index';
import { PlugLogin, StoicLogin, NFIDLogin, IdentityLogin } from "ic-auth";
import { createActor as ledgerActor, idlFactory as TokenIdl } from "../../../../declarations/ckbtc/index";
import {  idlFactory as ckETHIdlFactory } from "../../../../declarations/cketh/index";
import { DummyDataTokens } from '../../TextData';
// import { PlugMobileProvider } from '@funded-labs/plug-mobile-sdk'


const AuthContext = createContext();

export const useAuthClient = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [authClient, setAuthClient] = useState(null);
  const [identity, setIdentity] = useState(null);
  const [principal, setPrincipal] = useState(null); // Holds Principal object
  const [backendActor, setBackendActor] = useState(null);
  const [balance, setBalance] = useState(null);
  const [provider, setProvider] = useState(null); // Keep track of the provider
  // const isMobile = PlugMobileProvider.isMobileBrowser()
  
  useEffect(() => {

    AuthClient.create().then((client) => {
      setAuthClient(client);
    });

    
   
  }, []);

  useEffect(() => {
    if (authClient) {
      updateClient(authClient);
    }
  }, [authClient]);

  const login = async (selectedProvider) => {
    if (!authClient) {
      console.error("AuthClient not initialized yet.");
      return;
    }
    try {
      setProvider(selectedProvider); // Set the provider
    
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

      if (selectedProvider === "plug") {
        // Plug login
        // if (isMobile) {
        //   const provider = new PlugMobileProvider({
        //     debug: true, // If you want to see debug logs in console
        //     walletConnectProjectId: '6e2de4a3633b8ad436730aea43901ef3', // Project ID from WalletConnect console
        //     window: window,
        //   })
        //   // setProvider(provider)
        //   provider.initialize().catch(console.log)
        //   setProvider(provider)

        //   if (!provider.isPaired()) {
        //     provider.pair().catch(console.log)
        //   }

         
        //     const agent = await provider.createAgent({
        //       host: 'https://icp0.io',
        //       targets: [whitelist], // List of canister you are planning to call
        //     })
        
        // const backendActor = createActorBackend(process.env.CANISTER_ID_VALUESWAP_BACKEND, {agent:agent});
        // const principal = agent.getPrincipal()
        // setBackendActor(backendActor);
        // setPrincipal(principal);
        // setIsAuthenticated(true);
        // }
        // Collect all canister IDs you need to whitelist
       
        // Ensure all canister IDs are valid
        if (uniqueWhitelist.includes(undefined) || uniqueWhitelist.includes('')) {
          console.error("One or more canister IDs are undefined or empty. Please check your environment variables.");
          console.error("Whitelist:", uniqueWhitelist);
          return;
        }
  
        // Check if Plug is installed
        if (!window.ic || !window.ic.plug) {
          console.error("Plug wallet is not installed.");
          return;
        }
  
        // Request connection with the necessary whitelist
        const result = await window.ic.plug.requestConnect({
          whitelist: uniqueWhitelist,
          host: process.env.DFX_NETWORK === "local" ? 'http://localhost:3000' : 'https://mainnet.dfinity.network', // or your local network
          
        });
  
        if (!result) {
          console.error("User denied the connection.");
          return;
        }
        console.log("result o f infinity",result)
        // After connecting, the agent should be initialized
        if (!window.ic.plug.agent) {
          console.error("Plug agent is not initialized.");
          return;
        }
  
        // Get the principal via the agent
        const principal = await window.ic.plug.agent.getPrincipal();
        console.log("Principal:", principal.toString());
        setPrincipal(principal); // Store the Principal object
        setIsAuthenticated(true);
  
        // Create the backend actor using Plug's createActor method
        const backendActor = await window.ic.plug.createActor({
          canisterId: process.env.CANISTER_ID_VALUESWAP_BACKEND,
          interfaceFactory: idlFactory, // Ensure idlFactory is imported correctly
        });
        setBackendActor(backendActor);
  
        // Plug does not expose identity directly
        setIdentity(null);
  
        console.log("Plug login successful.");
      }else if( selectedProvider === "bitfinityWallet"){ 
        // Check if bitfinityWallet is installed
        if (!window.ic || !window.ic.infinityWallet) {
          console.error("bitfinityWallet wallet is not installed.");
          return;
        }
        const result = await window.ic.infinityWallet.requestConnect({
          whitelist: uniqueWhitelist,
          host: process.env.DFX_NETWORK === "local" ? 'http://localhost:3000' : 'https://mainnet.dfinity.network',
           // or your local network
          
        });
        console.log("result o f infinity",result)
        if (!result) {
          console.error("User denied the connection.");
          return;
        }
  
        // After connecting, the agent should be initialized
        // if (!window.ic.infinityWallet.agent) {
        //   console.error("bitfinityWallet agent is not initialized.");
        //   return;
        // }
  
        // Get the principal via the agent
        const principal = await window.ic.infinityWallet.getPrincipal();
        console.log("Principal:", principal.toString());
        setPrincipal(principal); // Store the Principal object
        setIsAuthenticated(true);
  
        // Create the backend actor using Plug's createActor method
        const backendActor = await window.ic.infinityWallet.createActor({
          canisterId: process.env.CANISTER_ID_VALUESWAP_BACKEND,
          interfaceFactory: idlFactory,
          host: process.env.DFX_NETWORK === 'local' ? 'http://localhost:3000' : 'https://mainnet.dfinity.network', // Ensure idlFactory is imported correctly
        });
        setBackendActor(backendActor);
  
        // Plug does not expose identity directly
        setIdentity(null);
  
        console.log("bitfinityWallet login successful.");

      }else if (selectedProvider === "stoic") {
        // Stoic login
        const userObject = await StoicLogin();
        const identity =   userObject.agent._identity; 
        // console.log("identity", StoicLogin())// StoicLogin returns identity
        const principal = await userObject.principal;
        console.log("principal stoic", principal, userObject)
        setPrincipal(principal); // Store the Principal object
        setIdentity(identity);
        setIsAuthenticated(true);
  
        const backendActor = createActorBackend(
          process.env.CANISTER_ID_VALUESWAP_BACKEND,
          { agentOptions: { identity } }
        );
        setBackendActor(backendActor);
  
        console.log("Stoic login successful.");
      } else if (selectedProvider === "nfid" || selectedProvider === "ii") {
        // NFID and II login
        const identityProvider =
          selectedProvider === "nfid"
            ? 'https://nfid.one/authenticate/'
            : 'https://identity.ic0.app/#authorize';
  
        await authClient.login({
          identityProvider,
          onSuccess: async () => {
            await updateClient(authClient);
            console.log(`${selectedProvider} login successful.`);
          },
          onError: (error) => {
            console.error(`${selectedProvider} login error:`, error);
          },
        });
      }
    } catch (error) {
      console.error("Login error:", error);
    }
  };
  

  const logout = async () => {
    if (!authClient) {
      console.error("AuthClient not initialized.");
      return;
    }
    try {
      if (provider === "plug") {
        // Plug logout logic (if any)
        console.log("window.ic", window.ic)
        // Plug does not have a logout method, but you can disconnect
        await window.ic.plug.disconnect();
      } else if (provider === "stoic") {
        // Stoic logout
        await authClient.logout();
      } else if (provider === "nfid" || provider === "ii") {
        // NFID and II logout
        await authClient.logout();
      }

      setIsAuthenticated(false);
      setIdentity(null);
      setPrincipal(null);
      setBackendActor(null);
      setProvider(null);
      console.log("Logout successful.");
    } catch (error) {
      console.error("Logout error:", error);
    }
  };

  const updateClient = async (client) => {
    const isAuthenticated = await client.isAuthenticated();
    setIsAuthenticated(isAuthenticated);
    const identity = client.getIdentity();
    setIdentity(identity);

    const principal = identity.getPrincipal();
    setPrincipal(principal); // Store the Principal object

    const backendActor = createActorBackend(
      process.env.CANISTER_ID_VALUESWAP_BACKEND,
      { agentOptions: { identity } }
    );
    setBackendActor(backendActor);
  };

  const createTokenActor = async (canisterID) => {
    if (provider === "plug") {
      // For Plug, use Plug's API to create the actor
      const tokenActor = await window.ic.plug.createActor({
        canisterId: canisterID,
        interfaceFactory: TokenIdl, // Ensure TokenIdl is imported correctly
      });
      return tokenActor;
    } else {
      // For other providers
      const tokenActor = ledgerActor(canisterID, { agentOptions: { identity } });
      return tokenActor;
    }
  };



  const getBalance = useCallback(async (canisterId) => {
  
    if (provider === "plug") {
        try { 
            const tokenActor = await window.ic.plug.createActor({
                canisterId: canisterId,
                interfaceFactory: TokenIdl, // Ensure TokenIdl matches the canister's IDL
              });
    
            console.log("Token actor:", tokenActor);
    
            // const ownerPrincipal = typeof principal === 'string' ? Principal.fromText(principal) : principal;
    
            const balance = await tokenActor.icrc1_balance_of({ owner: principal, subaccount: [] });
            console.log("Token balance:", balance);
             setBalance(balance);
             return balance;
          } catch (error) {
            console.error("Error fetching balance from Plug:", error);
            return null;
          }
    } else {
      // Handle other providers
      try {
        // Assuming you have a function to create an actor for other providers
        const actor = await createTokenActor(canisterId);
  
        const ownerPrincipal = typeof principal === 'string' ? Principal.fromText(principal) : principal;
     
        const balance = await actor?.icrc1_balance_of({ owner: ownerPrincipal,  subaccount: []});
        console.log("Balance:", balance.toString());
        return balance;
      } catch (error) {
        console.error("Error fetching balance from other provider:", error);
        return null;
      }
    }
  }, [provider, principal]);
  
  return {
    isAuthenticated,
    login,
    logout,
    authClient,
    identity,
    principal,
    backendActor,
    getBalance,
    createTokenActor,
    balance,
  };
};

export const AuthProvider = ({ children }) => {
  const auth = useAuthClient();

  if (!auth.authClient || (!auth.backendActor && auth.isAuthenticated)) {
    return null; // Or render a loading indicator
  }

  return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
};

export const useAuth = () => useContext(AuthContext);

















// 2nd
// import React, { createContext, useContext, useEffect, useState } from "react";
// import { PlugLogin, StoicLogin, NFIDLogin, IdentityLogin, Types, CreateActor } from 'ic-auth';
// // import { idlFactory,createActor} from "../../declarations/loginme_backend/index";
// import { createActor, idlFactory } from "../../../../declarations/ckbtc/index"
// import { Principal } from "@dfinity/principal";
// import { AuthClient } from "@dfinity/auth-client";
// import { Actor } from "@dfinity/agent";

// const AuthContext = createContext();
// const canisterID = process.env.CANISTER_ID_CKBTC;
// const whitelist = [process.env.CANISTER_ID_CKBTC];

// export const useAuthClient = () => {
//     const [isAuthenticated, setIsAuthenticated] = useState(false);
//     const [principal, setPrincipal] = useState(null);
//     const [backendActor, setBackendActor] = useState(createActor(canisterID));
//     const [identity, setIdentity] = useState(null);
//     const [authClient, setAuthClient] = useState(null);
//     const [balance, setBalance] = useState(null)
//     const [userObjects,  setUserObjects] = useState()
//     useEffect(() => {
//         const initializeAuthClient = async () => {
//             const client = await AuthClient.create();
//             setAuthClient(client);
//             // 
//             if (await client.isAuthenticated()) {
//                 const identity = client.getIdentity();
//                 const principal = identity.getPrincipal();
//                 const actor =  createActor(canisterID, { agentOptions: { identity } });
//                 getBalance(principal, actor)
//                 setIsAuthenticated(true);
//                 setPrincipal(principal);
//                 setIdentity(identity);
//                 setBackendActor(actor);
//             }
//         };
//         // 
//         initializeAuthClient();
//     }, []);
//     // 
//     const login = async (provider) => {
//         if (authClient) {
//             let userObject = {
//                 principal: "Not Connected.",
//                 agent: undefined,
//                 provider: "N/A",
//             };
//             if (provider === "Plug") {
//                 userObject = await PlugLogin(whitelist);
//                 console.log("userobj", userObject)
//             } else if (provider === "Bitfinity") {
//                 userObject = await infinityLogin();
//             } else if (provider === "NFID") {
//                 userObject = await NFIDLogin();
//             } else if (provider === "Identity") {
//                 userObject = await IdentityLogin();
//             }
//             console.log("userObject", userObject);
//             const identity = await userObject?.agent?._identity;
//             const principal = Principal.fromText(userObject?.principal);
           
//             // const ledgerActor = Actor.createActor(ledgerIdl, { userObject.agent, canisterId: process.env.CANISTER_ID_CKBTC });
//             setUserObjects(userObject)
//             setIsAuthenticated(true);
//             setPrincipal(principal);
//             setIdentity(identity);
//             // 
//             // await authClient.login({
//             //     identity,
//             //     onSuccess: () => {
//             //         setIsAuthenticated(true);
//             //         setPrincipal(principal);
//             //         setIdentity(identity);
//             //     },
//             // });
//         }
//     };
//     // 
//     const createTokenActor = async (canisterID) => {
//         const actor = await createActor(canisterID, { agentOptions: { identity } })
//         return actor;

//     }
//     const logout = async () => {
//         if (authClient) {
//             await authClient.logout();
//             setIsAuthenticated(false);
//             setPrincipal(null);
//             setIdentity(null);
//         }
//     };
//     // 
//     const reloadLogin = () => {
//         return new Promise(async (resolve, reject) => {
//             try {
//                 if (isAuthenticated() && ((await getIdentity().principal().isAnonymous()) === false)) {
//                     updateClient(authClient);
//                     resolve(AuthClient);
//                 }
//             } catch (error) {
//                 reject(error);
//             }
//         })
//     };

//    const getBalance = async (principal, canisterId) =>{
//     const actor = await createTokenActor(canisterId)
//     const balance = await actor.icrc1_balance_of({ owner: principal, subaccount: [] })
//     setBalance(balance)
//     console.log("initialActor", actor)
//     return balance;
//    }


//     // 
//     return {
//         // 
//         login,
//         logout,
//         principal,
//         isAuthenticated,
//         setPrincipal,
//         createTokenActor,
//         backendActor,
//         identity,
//         getBalance,
//         balance
//     };
// };
// // 
// export const AuthProvider = ({ children }) => {
//     const auth = useAuthClient();
//     return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
// };
// // 
// export const useAuth = () => useContext(AuthContext);







// import { useIdentityKit } from '@nfid/identitykit/react';
// import React, { createContext, useContext, useState, useEffect } from 'react';
// import { createActor as createActorBackend, idlFactory } from '../../declarations/h1_backend/index';
// import { Actor } from '@dfinity/agent';

// const AuthContext = createContext();

// export const useAuthClient = () => {
//   const { agent } = useIdentityKit();
//   const {
//     isInitializing,
//     user,
//     isUserConnecting,
//     icpBalance,
//     signer,
//     identity,
//     delegationType,
//     accounts,
//     connect,
//     disconnect,
//     fetchIcpBalance,
//   } = useIdentityKit();

//   const [userAuthenticated, setUserAuthenticated] = useState(false);
//   const [principal, setPrincipal] = useState(null);
//   const [backendActor, setBackendActor] = useState(null);
//   const [balance, setBalance] = useState(null);

//   const login = async () => {
//     await connect();
//     if (user) {
//       setUserAuthenticated(true);
//       setPrincipal(identity?._principal.toString());
//     } else {
//       setUserAuthenticated(false);
//     }
//   };

//   const logout = () => {
//     disconnect();
//     setUserAuthenticated(false);
//   };

//   // Initialize backend actor when agent or principal changes
//   useEffect(() => {
//     if (agent && process.env.CANISTER_ID_H1_BACKEND) {
//       const newBackendActor =  Actor.createActor(idlFactory, {
//         agent,
//         canisterId: process.env.CANISTER_ID_H1_BACKEND,
//       });
//       setBackendActor(newBackendActor);
//     }
    
//   }, []);

//   const createTokenActor = async (canisterId) => {
//     return Actor.createActor(idlFactory, {
//       agent,
//       canisterId: canisterId,
//     });
//   };

//   const getBalance = async (canisterId) => {
//     try {
//       const tokenActor = await createTokenActor(canisterId);
//       const balance = await tokenActor.icrc1_balance_of({ owner: principal, subaccount: [] });
//       setBalance(balance);
//       return balance;
//     } catch (error) {
//       console.error("Failed to fetch balance:", error);
//       return null;
//     }
//   };


//   return {
//     isInitializing,
//     isAuthenticated: user,
//     isUserConnecting,
//     icpBalance,
//     signer,
//     identity,
//     delegationType,
//     accounts,
//     backendActor,
//     createTokenActor,
//     agent,
//     login,
//     logout,
//     fetchIcpBalance,
//     principal,
//     getBalance,
//     balance,
//   };
// };

// export const AuthProvider = ({ children }) => {
//   const auth = useAuthClient();

//   return (
//     <AuthContext.Provider value={auth}>
//       {children}
//     </AuthContext.Provider>
//   );
// };

// export const useAuth = () => useContext(AuthContext);
