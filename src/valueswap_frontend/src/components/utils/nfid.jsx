import { createContext } from "react";

const {
    isInitializing,
    user,
    isUserConnecting,
    icpBalance,
    signer,
    identity,
    delegationType,
    accounts,
    connect,
    disconnect,
    fetchIcpBalance,
} = useIdentityKit()
import { useAgent } from "@nfid/identitykit/react"

const AuthContext = createContext()

export const useAuthClient = () => {
    const agent = useAgent()

}

export const AuthProvider = ({ children }) => {
    const auth = useAuthClient();

    if (disconnect) {
        return null; // Or render a loading indicator
    }

    return <AuthContext.Provider value={auth}>{children}</AuthContext.Provider>;
};

export const useAuth = () => useContext(AuthContext);