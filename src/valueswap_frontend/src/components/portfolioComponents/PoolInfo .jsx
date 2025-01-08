import React, { useState, useEffect } from 'react'
import { useNavigate, useParams } from 'react-router-dom';
import { portfolioSampleData } from '../../TextData';
import PoolInfoBox from '../../displayBoxes/PoolInfoBox';
import GradientButton from '../../buttons/GradientButton'
import { PoolCompositions, Swapping, LiquidityOverview } from '../../tables'
import Echarts from './Echarts';
import WithdrawModel from '../../Modals/WithdrawModel';
import { useAuth } from '../utils/useAuthClient';
import PoolAttributes from './PoolAttributes';

const PoolInfo = () => {

  const { id } = useParams()
  const [currIndex, setCurrIndex] = useState(0)
  const [currentRang, setCurrentRange] = useState(0)
  const Heading = ['Pool Compositions', 'Swapping', 'Liquidiity Overview']
 const [openWithdraw, setOpenWithdraw] = useState(false)
 const [specificPool, setSpecificPool] = useState([])
 const {backendActor, getBalance} = useAuth()
 const navigate = useNavigate()
  useEffect(() => {
    console.log("pool id", id)
    console.log("getBalance", getBalance("bkyz2-fmaaa-aaaaa-qaaaq-cai"));
    const poolData = async () =>{
     const pool = await backendActor.get_specific_pool_data(id)
     const poolDataArray = pool.Ok[0].pool_data;
     console.log("specific pool data array", poolDataArray);
     setSpecificPool(poolDataArray);
    }
    poolData()
  }, [id])


  const selectRang = [
    "1D",
    "1W",
    "1M",
    "1Y",
    "All Time"
  ]

  return (
    <div className=' max-w-[1200px] mx-auto relative '>
      <div className='w-full text-white mt-12 z-20 sm:px-8 '>

        <div className='flex flex-col justify-between  p-2  py-6  rounded-t-lg'>
        <div className='flex flex-col justify-between  p-2  py-6  rounded-t-lg'>
          <div className='flex justify-between items-center  mx-2  md:ml-8'>
            <div className='font-gilroy text-base md:text-3xl font-medium flex items-center gap-4'>
              <div className='flex gap-1 sm:gap-2'>
                {
                  specificPool?.map((token, index) => (
                    <div key={index}>
                      <div className='bg-[#3D3F47] p-1 rounded-lg'>
                        <img src={token.image} alt="token-image" className='w-6 h-6 md:w-10 md:h-10' />
                      </div>
                    </div>
                  ))
                }
              </div>
              <div className='flex items-center'>
                <span  >{specificPool?.[0]?.token_name}</span>
                {
                  specificPool?.slice(1).map((token, index) => (
                    <div key={index} className=''>
                      <span className='mx-0.5'>/</span>
                      {token.token_name}
                    </div>
                  ))
                }
                {/* <span className='mx-1'>:  :</span> */}

                {/* <span>{specificPool?.[0]?.weight * 100}</span> */}
                {
                  specificPool?.slice(1).map((token, index) => (
                    <div key={index} className=''>
                      <span className='mx-0.5'>/</span>
                      {token.weight *100}
                    </div>
                  ))
                }
              </div>
            </div>
          </div>
          <div className='flex flex-col lg:flex-row justify-between w-full gap-11 mx-auto mt-7'>
            <div className='min-w-[320px] flex flex-col justify-around items-start backdrop-blur-[32px] rounded-lg p-4 border border-white'>
              <PoolInfoBox Heading={'TVL'} Data={`$ ${specificPool?.PoolMetaData?.TwentyFourHourVolume.toLocaleString('en-US')}`} />
              <PoolInfoBox Heading={'Fees (24h)'} Data={`$ ${specificPool?.PoolMetaData?.TwentyFourHourFees.toLocaleString('en-US')}`} />
              <PoolInfoBox Heading={'Weekly Incentives'} Data={`$ ${specificPool?.PoolMetaData?.PoolValue.toLocaleString('en-US')}`} />
              {/* <div className='w-full sm:w-auto flex gap-4 h-20 lg:h-48 justify-center'>
                <PoolInfoBox Heading={'Pool Value'} Data={`$ ${specificPool?.PoolMetaData?.PoolValue.toLocaleString('en-US')}`} />
                <PoolInfoBox Heading={'24H_Fees'} Data={`$ ${specificPool?.PoolMetaData?.TwentyFourHourFees.toLocaleString('en-US')}`} />
              </div>
              <div className='w-full sm:w-auto flex gap-4 h-20 lg:h-48 justify-center'>
                <PoolInfoBox Heading={'24H_Pool Volume'} Data={`$ ${specificPool?.PoolMetaData?.TwentyFourHourVolume.toLocaleString('en-US')}`} />
                <PoolInfoBox Heading={'APR'} Data={`${specificPool?.PoolMetaData?.APRstart}% - ${specificPool?.PoolMetaData?.APRend}%`} />
              </div> */}
            </div>
            <div className=' lg:w-[59%] p-4 text-white border border-white backdrop-blur-[32px] rounded-lg'>
              {/* pool info chart here in this div */}
              <div>
                <div className='flex justify-between'>
                  <p className=' sm:text-3xl text-white font-semibold'>$125,625,175</p>
                  <div className='flex flex-col gap-y-2'>
                    <div className='flex justify-around gap-x-4 text-sm '>
                      <div>
                        <p>Volumes in Past-</p>
                        <hr className='border-[#4b4b4b]' />
                      </div>
                      <select name="" id="" className='bg-transparent backdrop-blur-sm text-white p-1 border-[1px] border-white focus:outline-none rounded-md'>
                        <option value="volume">Volume</option>
                        <option value="24hr">24hr Vol</option>
                      </select>
                    </div>
                    <div className='flex gap-x-2 text-sm'>
                      {
                        selectRang.map((rang, index) => 
                          <div className='flex flex-col items-center' key={index} onClick={() => {
                            setCurrentRange(index)
                          }}>
                            <p className='cursor-pointer'>{rang}</p>
                            <span className={`p-[2px] w-1 bg-[#F7931A]  ${currentRang === index ? 'visible' : 'invisible'}`}></span>
                          </div>
                        )
                      }

                    </div>
                  </div>
                </div>
                <Echarts />
              </div>
              <div>

              </div>
            </div>
          </div>

          <div className='flex md:flex-row flex-col items-center justify-between mt-2'>
          <div className='gap-2 font-gilroy flex items-center'>
            <span className='text-base leading-5 font-bold opacity-75 tracking-wide'>My Pool Balance:</span>
            <span className='mx-3 text-2xl font-normal leading-6'>${specificPool?.PoolMetaData?.PersonalPoolBalance.toLocaleString('en-US')}</span>
          </div>

         
          <div className='flex gap-3 md:gap-6 my-4'>
            <div>
              <GradientButton CustomCss={`text-xs md:text-base lg:text-base  lg:w-[150px] py-2`}>
                Swap Tokens
              </GradientButton>
            </div>
            <div>
              <GradientButton CustomCss={`text-xs md:text-base lg:text-base  lg:w-[150px] py-2`}>
                Add Liquidity
              </GradientButton>
            </div>
            <div onClick={()=> setOpenWithdraw(true)}>
              <GradientButton CustomCss={`text-xs md:text-base lg:text-base  lg:w-[150px] py-2`} onClick={()=> navigate("/valueswap/on-withdraw")}>
                Withdraw
              </GradientButton>
            </div>
          </div>
          </div>

          {/* <div className='font-gilroy font-medium text-base md:text-xl lg:text-2xl flex gap-3 md:gap-16 lg:gap-32 mx-4 lg:mx-10 mt-6'>
            {Heading.map((heading, index) => (
              <div className='flex flex-col justify-center text-center items-center gap-2 cursor-pointer' key={index}
                onClick={() => {
                  setCurrIndex(index)
                }}>
                <h1>{heading}</h1>
                <span className={`p-[1px]  bg-[#F7931A] w-full ${currIndex === index ? 'visible' : 'invisible'}`}></span>
              </div>
            ))}
          </div> */}

          <div className='w-full grid grid-cols-1 md:grid-cols-2 gap-4'>
            <PoolCompositions TableData={specificPool?.PoolData} />
            <PoolAttributes pool={specificPool}/>
          </div>
          {/* <div >
            {currIndex === 0 && <PoolCompositions TableData={specificPool?.PoolData} />}
            {currIndex === 1 && <Swapping id={Number(id)} />}
            {currIndex === 2 && <LiquidityOverview id={id} />}
          </div> */}



        </div>

      </div>
      {openWithdraw ? <WithdrawModel setOpenWithdraw={setOpenWithdraw} poolName={id}/> : ""}
    </div>
    </div>
  )
}

export default PoolInfo 
