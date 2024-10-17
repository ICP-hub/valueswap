import React, { useState } from 'react'
import BorderGradientButton from '../buttons/BorderGradientButton'
import GradientButton from '../buttons/GradientButton'
import { useAuth } from '../components/utils/useAuthClient'
import { toast } from 'react-toastify'
import CloseOutlinedIcon from '@mui/icons-material/CloseOutlined';
function WithdrawModel({ setOpenWithdraw }) {
  const [change, setChange] = useState(false)

  const {backendActor} = useAuth()

  const withdrawCoinHandler = async() =>{
    setChange(true)
    try {
      console.log(backendActor)
      const res =  await backendActor.burn_lp_tokens({pool_name: "cketh", amount: 33})
      if(res.Error){
        toast.error("we are fixing withdraw issue")
        setChange(false)
      }
      setChange(false)
      toast.success("successfully withdraw 🤞")
    } catch (error) {
      console.error(error)
    }
  }
  return (
    <div className='fixed inset-0 z-50 flex justify-center items-center bg-black bg-opacity-50 font-cabin'>
      <div className='relative w-10/12  md:w-2/5 lg:w-1/3 bg-[#010427] px-2 pt-4 h-5/6  sm:h-4/6 border-2 rounded-lg z-50'>
        <div className=' w-full flex  justify-end cursor-pointer  pr-4 z-50'>
          <button onClick={() => setOpenWithdraw(false)}>
            <CloseOutlinedIcon/>
          </button>
        </div>
        <div className='absolute top-[14%] bottom-0 left-0 right-0 m-auto z-10 flex flex-col gap-y-4 px-8'>
          <label htmlFor="number" className='text-lg font-fahkwang font-medium'>Input your LP</label>
          <ul className='text-sm font-light list-disc list-inside'>
            <li>First it show your coin respect to Lp you input.</li>
            <li>Then you can withdraw your coin.</li>
          </ul>
          <input type="number" name="lp" id="" className='rounded-2xl  bg-[#1e2021a1] p-2' />
          <div className='flex justify-center' onClick={()=> withdrawCoinHandler()}>

          <GradientButton CustomCss={`text-xs md:text-base lg:text-base  lg:w-[150px] py-2 px-2`}>
            {change ? "withdraw": "Fetch detail"}
          </GradientButton>
          </div>
          <div>
            <div className='flex justify-between'>
              <span>ckbtc</span>
              <span>2</span>
            </div>
            <div className='flex justify-between'>
              <span>cketh</span>
              <span>4</span>
            </div>
          </div>
          {/* <BorderGradientButton>
        fetch detail
      </BorderGradientButton> */}
        </div>
      </div>
    </div>
  )
}

export default WithdrawModel