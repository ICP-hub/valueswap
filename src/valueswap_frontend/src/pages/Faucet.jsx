import { Wallet } from 'lucide-react'
import React, { useState } from 'react'
import BorderGradientButton from '../buttons/BorderGradientButton'
import GradientButton from '../buttons/GradientButton'
import FaucetModal from '../Modals/FaucetModal'
import { useAuth } from '../components/utils/useAuthClient'

let tokens = [
    {
        imgUrl: "/image/ckBTC.svg",
        TokenName: "ckBTC",
        WalletBalance: "0"
    },
    {
        imgUrl: "/image/ckETH.svg",
        TokenName: "ckETH",
        WalletBalance: "0"
    },
]
const Faucet = () => {
    const [modelOpen, setModelOpen] = useState(false);
    const[selectFauce, setSelectFaucet] = useState([])
    const {isAuthenticated, backendActor} = useAuth()

   
    console.log(selectFauce[0])
    return (
        <section className='mt-16 max-w-[1200px] mx-auto space-y-4 font-gilroy px-8 xl:px-0 relative'>
            <div>
                <p className='text-[#FFFFFFBF]'>With our testnet Faucet you can receive free assets to test the Dfinance Protocol. Make sure to switch your wallet provider to the appropriate testnet network, select desired asset, and click ‘Faucet’ to get tokens transferred to your wallet. The assets on our testnet are not “real,” meaning they have no monetary value.</p>
            </div>
          {isAuthenticated ? <div>
            <div className='font-semibold text-xl'>Test Assets</div>
            <table className='w-full '>
                <tr className='flex  justify-between items-center text-[#FFFFFFBF]'>
                    <td>Assest</td>
                    <td>Wallet Balance</td>
                    <td></td>
                </tr>
                {
                    tokens.map((token, id) => (
                        <tr key={id} className='grid grid-cols-3 py-4 items-center border-b-2 cursor-pointer '>
                            <td className='flex gap-x-2 items-center'>
                                <img src={token.imgUrl} alt="token image" className='w-10 h-10' />
                                <p className='font-semibold'>{token.TokenName}</p>
                            </td>
                            <td className='flex  flex-col items-center'>
                                <p>{token.WalletBalance}</p>
                                <p>${0}</p>
                            </td>
                            <td className='text-end'>

                                <GradientButton onClick={() => {
                                    setModelOpen((prev) => !prev)
                                    setSelectFaucet([{imgUrl:token.imgUrl}, {TokenName: token.TokenName}])
                                } }>
                                    Faucet
                                </GradientButton>


                            </td>

                        </tr>
                    ))
                }
            </table>
          {modelOpen && <FaucetModal setModelOpen={setModelOpen} TokenName={selectFauce[1]} imgUrl={selectFauce[0]}/>}
                </div>: 
                
                <div className='my-auto w-full text-center '>
                    <h2 className='text-2xl'> Please, connect your wallet</h2>
                    <p> Please connect your wallet to get free testnet assets</p>
                </div>
                }
        </section>
    )
}

export default Faucet