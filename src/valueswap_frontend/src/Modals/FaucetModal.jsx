import React from 'react'
import { X } from 'lucide-react'
import GradientButton from '../buttons/GradientButton'
const FaucetModal = ({setModelOpen, imgUrl, TokenName}) => {
  console.log("TokenName", TokenName)
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
         <input type="number" className='bg-transparent p-2 border-2 border-[#3c3f44] active:border-[#3c3f44] focus:outline-none rounded-md'/>
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

          <GradientButton CustomCss={` w-full md:w-full`}>
         Faucet {TokenName.TokenName}
        </GradientButton>
        </div>
      </div>
      </div>

  )
}

export default FaucetModal