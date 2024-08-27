import { AuthClient } from "@dfinity/auth-client";
import React, { createContext, useContext, useEffect, useState } from "react";
import {
    createActor as createActorBackend, 
} from '../../../../declarations/valueswap_backend/index';
// import { Actor, HttpAgent } from "@dfinity/agent";
import { createActor as ledgerActor, idlFactory as TokenIdl} from "../../../../declarations/ckbtc_ledger/index"


const AuthContext = createContext();

const defaultOptions = {
    /**
     *  @type {import("@dfinity/auth-client").AuthClientCreateOptions}
     */
    createOptions: {
        // idleOptions: {
        //   // Set to true if you do not want idle functionality
        //   disableIdle: true,
        // },
        idleOptions: {
            idleTimeout: 1000 * 60 * 30, // set to 30 minutes
            disableDefaultIdleCallback: true, // disable the default reload behavior
        },
    },
    /**
     * @type {import("@dfinity/auth-client").AuthClientLoginOptions}
     */
    loginOptionsIcp: {
        identityProvider:
            process.env.DFX_NETWORK === "ic"
                ? "https://identity.ic0.app/#authorize"
                : `http://rdmx6-jaaaa-aaaaa-aaadq-cai.localhost:4943/`,
    },
    loginOptionsnfid: {
        identityProvider:
            process.env.DFX_NETWORK === "ic"
                ? `https://nfid.one/authenticate/?applicationName=my-ic-app#authorize`
                : `https://nfid.one/authenticate/?applicationName=my-ic-app#authorize`
    },
};

/**
 *
 * @param options - Options for the AuthClient
 * @param {AuthClientCreateOptions} options.createOptions - Options for the AuthClient.create() method
 * @param {AuthClientLoginOptions} options.loginOptions - Options for the AuthClient.login() method
 * @returns
 */
export const useAuthClient = (options = defaultOptions) => {
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [authClient, setAuthClient] = useState(null);
    const [identity, setIdentity] = useState(null);
    const [principal, setPrincipal] = useState(null);
    const [balance, setBalance] = useState(null);
   const [backendActor, setBackendActor] = useState(null)
    useEffect(() => {
        // Initialize AuthClient
        AuthClient.create(options.createOptions).then((client) => {
            setAuthClient(client);
        });
    }, []);
 const backendCanisterId = process.env.CANISTER_ID_VALUESWAP_BACKEND;
    const login = (val) => {
        return new Promise(async (resolve, reject) => {
            try {
                if (authClient.isAuthenticated() && ((await authClient.getIdentity().getPrincipal().isAnonymous()) === false)) {
                    console.log("click to login")
                    const backendActor = await createActorBackend(backendCanisterId, { agentOptions: { identity: identity } });
                    console.log("backendactor creat",backendActor )
                    setBackendActor(backendActor);
                    updateClient(authClient);
                    resolve(AuthClient);
                } else {
                    let opt = val === "Icp" ? "loginOptionsIcp" : "loginOptionsnfid"
                    authClient.login({
                        ...options[opt],
                        onError: (error) => reject(error),
                        onSuccess: () => {
                            updateClient(authClient);
                            resolve(authClient);
                        },
                    });
                }
            } catch (error) {
                reject(error);
            }
        })
    };

    // const createTokenActor = (canisterId) => {
    //     if(!canisterId){
    //         throw new Error("Canister ID is undefined");
    //     }
    //     const agent = new HttpAgent();
    //     let tokenActor = createActor(TokenIdl, {
    //         agent,
    //         canisterId: canisterId,
    //     });
    //     return tokenActor;
    // };

    const reloadLogin = () => {
        return new Promise(async (resolve, reject) => {
            try {
                if (authClient.isAuthenticated() && ((await authClient.getIdentity().getPrincipal().isAnonymous()) === false)) {
                    updateClient(authClient);
                    resolve(AuthClient);
                }
            } catch (error) {
                reject(error);
            }
        })
    };

    async function updateClient(client) {
        const isAuthenticated = await client.isAuthenticated();
        setIsAuthenticated(isAuthenticated);
        const identity = client.getIdentity();
        setIdentity(identity);
        const principal = identity.getPrincipal();
        setPrincipal(principal);
        setAuthClient(client);
        // console.log(identity);
    }

  

    async function logout() {
        await authClient?.logout();
        await updateClient(authClient);
        setIsAuthenticated(false);
    }


    const createTokenActor = (canisterID) => {
        let tokenActor = ledgerActor(canisterID, { agentOptions: { identity: identity } })
        return tokenActor;
        
            }
        

    const canisterId =
        process.env.CANISTER_ID_CKETH_LEDGER

    // const actor = createActorBackend(canisterId, { agentOptions: { identity } });

const getBalance = async (principal, canisterId) =>{
    const actor = await createTokenActor(canisterId)
    const balance = await actor.icrc1_balance_of({ owner: principal, subaccount: [] })
    setBalance(balance)
    console.log("initialActor", actor)
    return balance;
   }


    return {
       login,
        logout,
        principal,
        isAuthenticated,
        setPrincipal,
        createTokenActor,
        identity,
        getBalance,
        balance,
        backendActor
    };
};

/**
 * @type {React.FC}
 */
export const AuthProvider = ({ children }) => {
    const auth = useAuthClient();
    return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
};

export const useAuth = () => useContext(AuthContext);

// import React, { createContext, useContext, useEffect, useState } from "react";
// import { PlugLogin, StoicLogin, NFIDLogin, IdentityLogin, Types, CreateActor } from 'ic-auth';
// // import { idlFactory,createActor} from "../../declarations/loginme_backend/index";
// import { createActor, idlFactory } from "../../../../declarations/ckbtc_ledger/index"
// import { Principal } from "@dfinity/principal";
// import { AuthClient } from "@dfinity/auth-client";
// import { Actor } from "@dfinity/agent";

// const AuthContext = createContext();
// const canisterID = process.env.CANISTER_ID_CKBTC_LEDGER;
// const whitelist = [process.env.CANISTER_ID_CKBTC_LEDGER];

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
           
//             // const ledgerActor = Actor.createActor(ledgerIdl, { userObject.agent, canisterId: process.env.CANISTER_ID_CKBTC_LEDGER });
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