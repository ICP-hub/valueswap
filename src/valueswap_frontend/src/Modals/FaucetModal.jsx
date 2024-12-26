<<<<<<< HEAD
import React, { useState } from 'react'
import { X } from 'lucide-react'
import GradientButton from '../buttons/GradientButton'
import { toast } from 'react-toastify'
import { useAuth } from '../components/utils/useAuthClient'
import { Principal } from '@dfinity/principal'
const FaucetModal = ({setModelOpen, imgUrl, TokenName}) => {
  const [faucetAmount, setFaucetAmount] = useState(0)
  const {createTokenActor, backendActor, principal, identity  } = useAuth();
  let faucetList = [{name: "ckBTC", CanisterId: process.env.CANISTER_ID_CKBTC},  {name:"ckETH", CanisterId: process.env.CANISTER_ID_CKETH}, {name:"LP token", CanisterId: process.env.CANISTER_ID_LP_LEDGER_CANISTER}];
  let ledger_canister_id = faucetList?.find((token) => token.name.toLowerCase() == TokenName.TokenName?.toLowerCase() )
   const depositeHandler = async(scaledAmount) =>{
    let ledgerPrincipal = Principal.fromText(ledger_canister_id.CanisterId)
    console.log("called")
    const res = await backendActor?.faucet(ledgerPrincipal, principal, BigInt(scaledAmount) );
    console.log("res", res)
    if(res.Ok){
      toast.success("Transfer Complete")
    }
   }

  console.log("TokenName", TokenName.TokenName)
=======
import React from 'react'
import { X } from 'lucide-react'
import GradientButton from '../buttons/GradientButton'
const FaucetModal = ({setModelOpen, imgUrl, TokenName}) => {
  console.log("TokenName", TokenName)
>>>>>>> 26ad3fc (add faucet)
  return (
  <div className='fixed top-[12%] left-0 w-full h-full bg-opacity-50 backdrop-blur-sm'>
      <div className='  max-w-[480px] h-[300px] mt-36 mx-auto  bg-[#182030] rounded-lg  p-8'>
        <div className='flex justify-between'>
          <h1 className='text-2xl font-semibold'>
            Faucet ICP
          </h1>
          <X onClick={()=> setModelOpen(false)} className='cursor-pointer'/>
        </div>
        <div className='flex flex-col justify-center h-full gap-y-4'>
        <div>Transaction overview</div>
        <div className='flex justify-between'>
         <div className='my-auto'>
<<<<<<< HEAD
         <input type="number" className='bg-transparent p-2 border-2 border-[#3c3f44] active:border-[#3c3f44] focus:outline-none rounded-md' onChange={(e)=> setFaucetAmount(e.target.value)}/>
=======
         <input type="number" className='bg-transparent p-2 border-2 border-[#3c3f44] active:border-[#3c3f44] focus:outline-none rounded-md'/>
>>>>>>> 26ad3fc (add faucet)
         </div>
          <div className='space-y-4'>
            <div className='flex gap-x-2 items-center'>
              <img src={imgUrl.imgUrl} alt="" className='w-10 h-10'/>
              <p className='font-medium text-lg'>{TokenName.TokenName}</p>
            </div>
            <div className='flex gap-x-1 bg-[#37415173] p-1 px-2 rounded-md'>
              <p className='text-gray-300'>Approx.</p>
              <p>$500 Max</p>
            </div>
          </div>
        </div>

<<<<<<< HEAD
          <GradientButton CustomCss={` w-full md:w-full`} onClick={() =>depositeHandler(faucetAmount)}>
=======
          <GradientButton CustomCss={` w-full md:w-full`}>
>>>>>>> 26ad3fc (add faucet)
         Faucet {TokenName.TokenName}
        </GradientButton>
        </div>
      </div>
      </div>

  )
}

export default FaucetModal