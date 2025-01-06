import { Wallet } from 'lucide-react'
import React, { useEffect, useState } from 'react'
import BorderGradientButton from '../buttons/BorderGradientButton'
import GradientButton from '../buttons/GradientButton'
import FaucetModal from '../Modals/FaucetModal'
import { useAuth } from '../components/utils/useAuthClient'

let tokens = [
    {
        imgUrl: "/image/ckBTC.svg",
        TokenName: "ckBTC",
        WalletBalance: "0",
        CanisterId: process.env.CANISTER_ID_CKBTC
    },
    {
        imgUrl: "/image/ckETH.svg",
        TokenName: "ckETH",
        CanisterId: process.env.CANISTER_ID_CKETH
    },
    {
        imgUrl: "/image/ckETH.svg",
        TokenName: "LP token",
        CanisterId: process.env.CANISTER_ID_LP_LEDGER_CANISTER
    },
]

const Faucet = () => {
    const [modelOpen, setModelOpen] = useState(false);
    const [selectFaucet, setSelectFaucet] = useState([]);
    const [balances, setBalances] = useState({});
    const { isAuthenticated, backendActor, getBalance } = useAuth();
    let balance;
    // Fetch the balance for each token when the component is mounted or when `isAuthenticated` changes
    useEffect(() => {
        if (isAuthenticated) {
            const fetchBalances = async () => {
                let newBalances = {};
                for (const token of tokens) {
                    try {
                        const balance = await getBalance(token.CanisterId);
                        newBalances[token.TokenName] = balance;
                    } catch (error) {
                        console.error(`Error fetching balance for ${token.TokenName}`, error);
                        newBalances[token.TokenName] = 0; // Fallback to 0 if there was an error
                    }
                }
                setBalances(newBalances);
            };

            fetchBalances();
        }
    }, [isAuthenticated, modelOpen]);

    // Handle display of balance in table
    const displayBalance = (tokenName) => {

        return balances[tokenName] !== undefined ? Number(balances[tokenName]) : "Loading...";
    };

    return (
        <section className=' max-w-[1200px] mx-auto space-y-10 font-gilroy px-8 xl:px-0 relative pb-12'>
         
          <h1 className='text-center text-3xl pt-10'>Faucet</h1>
          
          
            <div className='w-full '>
                <p className='text-[#FFFFFFBF] max-w-[1000px] text-center mx-auto '>
                    With our testnet Faucet, you can receive free assets to test the Dfinance Protocol.
                    Make sure to switch your wallet provider to the appropriate testnet network, select the desired asset, and click ‘Faucet’ to get tokens transferred to your wallet.
                    The assets on our testnet are not “real,” meaning they have no monetary value.
                </p>
            </div>

            {isAuthenticated ? (
                <div className='h-screen  bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-10 border  border-[#FFFFFF66] rounded-2xl p-8 mb-8'>
                    {/* <div className='font-semibold text-xl'>Test Assets</div> */}
                    <table className='w-full '>
                        <thead>
                            <tr className='flex justify-between items-center text-[#FFFFFFBF] border-b-[2px] py-4'>
                                <th>Asset</th>
                                <th>Wallet Balance</th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            {tokens.map((token, id) => (
                                <tr key={id} className='grid grid-cols-3 py-4 items-center border-b-[0.5px] cursor-pointer '>
                                    <td className='flex gap-x-2 items-center'>
                                        <img src={token.imgUrl} alt="token image" className='w-10 h-10' />
                                        <p className='font-semibold'>{token.TokenName}</p>
                                    </td>
                                    <td className='flex flex-col items-center'>
                                       
                                        <p>{ balances[token.TokenName] !== undefined ? Number(balances[token.TokenName])/100000000 : "Loading..."}</p>
                                        <p>${0}</p>
                                    </td>
                                    <td className='text-end'>
                                        <GradientButton
                                            onClick={() => {
                                                setModelOpen((prev) => !prev);
                                                setSelectFaucet([{ imgUrl: token.imgUrl }, { TokenName: token.TokenName }]);
                                            }}
                                        >
                                            Faucet
                                        </GradientButton>
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                   
                </div>
            ) : (
                <div className='my-auto w-full text-center'>
                    <h2 className='text-2xl'> Please, connect your wallet</h2>
                    <p> Please connect your wallet to get free testnet assets</p>
                </div>
            )} 
            
            {modelOpen && <FaucetModal setModelOpen={setModelOpen} TokenName={selectFaucet[1]} imgUrl={selectFaucet[0]} />}
        </section>
    );
}

export default Faucet;
