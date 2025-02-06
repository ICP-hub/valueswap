import React, { useState, useEffect, useCallback, useMemo } from 'react'
import { useParams } from 'react-router-dom';
import { portfolioSampleData } from '../../TextData';
import PoolInfoBox from '../../displayBoxes/PoolInfoBox';
import GradientButton from '../../buttons/GradientButton'
import { PoolCompositions, Swapping, LiquidityOverview } from '../../tables'
import Echarts from '../portfolioComponents/Echarts';
import { IOSSwitch } from '../../buttons/SwitchButton';
import { convertTokenEquivalentUSD } from '../../utils';
import { useAuth } from '../utils/useAuthClient';

const RestTokens = [
  {
    ImagePath: "path/to/image1.png",
    ShortForm: "ETH",
    weights: 50,
    currencyAmount: 1000,
  },
  {
    ImagePath: "path/to/image2.png",
    ShortForm: "BTC",
    weights: 30,
    currencyAmount: 2000,
  },
];

const Result = {
  heading : 'Total',
  headingData : '$0.00',
  data : [
    {
      title : 'Total Pool value locked',
      value : '$125,165'
    },
    {
      title : 'LP Tokens',
      value : 0.0086
    },
    {
      title : 'Your pool share',
      value : '0.0001%',
    },
    {
      title : 'Gas fee',
      value : 0.00052
    }
  ]
}


const AddLiquidity = () => {

  const { id } = useParams()
  const [currIndex, setCurrIndex] = useState(0)
  const [currentRang, setCurrentRange] = useState(0)
  const [tokens, setTokens] = useState([])
  const [restTokens, setRestTokens] = useState([])
  const Heading = ['Pool Compositions', 'Swapping', 'Liquidiity Overview']
  const {backendActor,principal} = useAuth()

  useEffect(() => {
    console.log("pool id", id)
    getPoolData(id)
  }, [id,principal])

  useEffect(()=>{
    setRestTokens(()=>{
      const splittedTokenArr = tokens.slice(1)
      const restTT = splittedTokenArr.map(token=>{
        return {
            ImagePath: token.image,
            ShortForm: token.token_name.toUpperCase(),
            weights: parseFloat(token.weight),
            currencyAmount: 1000,
            balance : parseFloat(token.balance) / Math.pow(10,8) // Get decimals from backend
        }
      })
      return restTT
    })
  },[tokens])

  /**
   * Fetches the pool data from the backend
   * @param {string} pool_id
   * @returns {void}
   */
  const getPoolData = useCallback(async(pool_id)=>{
    try{
      const data = await backendActor.get_specific_pool_data(pool_id)
      if(data?.Ok){
        console.log("pool data", data.Ok)
        const pool_datas = data.Ok 
        setTokens(pool_datas[0].pool_data)
      }else{
        throw new Error(data.Err)
      }
    }catch(err){
      console.error("Error fetching pool data", JSON.stringify(err))
      setTokens([])
    }finally{
      console.log("done fetching pool data",tokens)
    }
  },[id])

  // let TokenData = portfolioSampleData.TableData[id]

  const selectRang = [
    "1D",
    "1W",
    "1M",
    "1Y",
    "All Time"
  ]

  const [optimizeEnable, setOptimizeEnable] = React.useState(true);
  const [initialTokenAmount, setInitialTokenAmount] = React.useState(0);
  const [equivalentUSD, setEquivalentUSD] = React.useState(0);
  const initialTokenBalance = 500; // Static value
  const initialTokenRef = React.useRef(null);
  const restTokensAmount = [100, 200]; // Example static data
  const restTokensRefs = React.useRef([]);

  const InitialToken = {
    ImagePath: "path/to/initial-image.png",
    ShortForm: "USDT",
    LongForm : "Tether",
    weights: 20,
    currencyAmount: 500,
  };

  const init = useMemo(()=>{
    const initialToken = tokens[0]
    console.log("II : ", initialToken)
    return {
      weights : initialToken?.weight.toString(),
      currencyAmount : 0,
      LongForm : "",
      ShortForm: initialToken?.token_name.toUpperCase(),
      ImagePath : initialToken?.image,
      balance : parseFloat(initialToken?.balance) / Math.pow(10,8) // Get Decimals from backend
    }
  },[id,principal,tokens])

  console.log("Init : \n",init,"\nRest :",restTokens)

  const fetchCurrPrice=useCallback(async()=>{
    const price = await convertTokenEquivalentUSD(init?.ShortForm, initialTokenAmount)
    return price
  },[id, initialTokenAmount])

  useEffect(()=>{
    fetchCurrPrice()
    .then((price)=>{
      console.log("price", price)
      setEquivalentUSD(price)
    })
  },[id, initialTokenAmount])

  const handleInput = (e, index) => {
    const value = parseFloat(e.target.value) || 0;
    if (index === 0) {
      setInitialTokenAmount(value);
    } else {
      restTokensAmount[index - 1] = value;
    }
  };

  const ButtonActive = true; // Static value
  const isAuthenticated = true; // Static value
  const AmountSelectCheck = true; // Static value
  const poolName = "ExamplePool"; // Static value

  return (
    <div className=''>
      <div className='w-max m-auto flex flex-col gap-4 p-3 sm:p-6 relative space-y-2'>
        <div className='flex gap-2 items-center justify-end w-full'>
          <p className='font-gilroy text-sm'>Auto optimize liquidity</p>
          <IOSSwitch sx={{ m: 1 }} defaultChecked onClick={() => setOptimizeEnable((prev) => !prev)} />
        </div>
        <div className='flex justify-between gap-12 items-center font-gilroy backdrop-blur-[32px] 
        md:px-6 px-3 md:py-8 py-4 rounded-xl border border-[#C0D9FF66]'>
          <div className='flex flex-col'>
            <div>
              <input
                className={`${initialTokenAmount > initialTokenBalance ? "text-red-500" : ""} font-normal leading-5 text-xl sm:text-3xl py-1 inline-block bg-transparent border-none outline-none`}
                type="number"
                min='0'
                value={isNaN(initialTokenAmount) ? "" : initialTokenAmount}
                ref={initialTokenRef}
                onChange={(e) => handleInput(e, 0)}
              />
            </div>
            <span className='text-sm sm:text-base font-normal'>
              ${init?.currencyAmount.toLocaleString() || 0}
            </span>
          </div>
          <div className='flex flex-col justify-center'>
            <div className='flex gap-3 items-center'>
              <img src={init?.ImagePath} alt="" className='h-3 aspect-square sm:h-4 transform scale-150 rounded-full' />
              <span className='text-base sm:text-2xl font-normal'>
                {init?.ShortForm}
              </span>
              <span className='text-sm sm:text-2xl font-normal'>•</span>
              <span className='py-1 px-2 sm:px-3'>
                {init?.weights} %
              </span>
            </div>
            <span className='inline-flex justify-center gap-2 w-full text-center font-normal leading-5 text-sm sm:text-base'>
              <p className={`${initialTokenAmount > initialTokenBalance ? "text-red-500" : ""}`}>{init?.balance.toLocaleString()} {init?.ShortForm}</p>
              <p className='text-white bg-gray-600 rounded-md px-2 h-fit text-[12px]'>Max</p>
            </span>
          </div>
        </div>

        <div className='flex flex-col gap-4'>
          {restTokens.map((token, index) => {
            const balance = token.balance; // Example static balance

            return (
              <div key={index}>
                <div className='flex justify-between items-center font-gilroy backdrop-blur-[32px] 
                md:px-6 px-3 md:py-8 py-4 rounded-xl border border-[#C0D9FF66]'>
                  <div className='flex flex-col'>
                    <div>
                      <input
                        className="font-normal leading-5 text-xl sm:text-3xl py-1 inline-block outline-none bg-transparent"
                        type="number"
                        min="0"
                        value={isNaN(restTokensAmount[index]) ? "" : restTokensAmount[index]}
                        placeholder="0"
                        ref={(el) => (restTokensRefs.current[index] = el)}
                        onChange={(e) => handleInput(e, index + 1)}
                      />
                    </div>
                    <span className='text-sm sm:text-base font-normal'>
                      ${token.currencyAmount?.toLocaleString() || "0"}
                    </span>
                  </div>
                  <div className='flex flex-col justify-center'>
                    <div className='flex gap-3 items-center'>
                      <img src={token.ImagePath} alt="" className='h-3 aspect-square sm:h-4 transform scale-150 rounded-full' />
                      <span className='text-sm sm:text-2xl font-normal'>
                        {token.ShortForm.toUpperCase()}
                      </span>
                      <span className='text-sm sm:text-2xl font-normal'>•</span>
                      <span className='py-1 px-2 sm:px-3'>
                        {token.weights} %
                      </span>
                    </div>
                    <span className='inline-flex justify-center gap-2 text-center font-normal leading-5 text-sm sm:text-base'>
                      {balance.toLocaleString()} {token.ShortForm.toUpperCase()}
                      <p className='text-white bg-gray-600 rounded-md px-2 h-fit text-[12px]'>Max</p>
                    </span>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
        <div
          className={`font-gilroy text-base font-medium`}
          onClick={() => {
            if (!isAuthenticated) {
              toast.warn('Please login first');
            } else if (!ButtonActive) {
              toast.warn('Please select all the coins');
            } else if (!AmountSelectCheck) {
              toast.warn('You do not have enough tokens.');
            } else {
              fetchPoolName(poolName);
              console.log("dispatched finished");
            }
          }}
        >
          <GradientButton CustomCss={`my-2 sm:my-4 w-full md:w-full ${ButtonActive ? 'opacity-100 cursor-pointer' : 'opacity-50 cursor-default'}`}>
            {initialTokenAmount == 0 ? 'Add Token Amount' : 'Add Liquidity'}
          </GradientButton>
        </div>
      {/* Info Content */}
        <table className='w-full font-gilroy'>
          <thead className='text-xl font-semibold'>
            <td>{Result.heading}</td>
            <td>{Result.headingData}</td>
          </thead>
          <tbody className='text-base'>
            {
              Result.data.map((data, index) => (
                <tr key={index}>
                  <td>{data.title}</td>
                  {
                    data.title === 'Gas fee' ? (
                      <td>{`${data.value} ${InitialToken.ShortForm} ( $${equivalentUSD} )`}</td>
                    ) : (
                      <td>{data.value.toLocaleString()}</td>
                    )
                  }
                </tr>
              ))
            }
          </tbody>
        </table>
      </div>
    </div>
  );

  // return (
  //   <div className=' max-w-[1200px] mx-auto h-screen relative '>

  //     <div className='w-full h-screen  text-white mt-12 z-20 sm:px-8 absolute'>

  //       <div className='flex flex-col justify-between bg-[#010427] p-2  py-6  rounded-t-lg mx-auto'>
  //         <div className='flex justify-between items-center  mx-2  md:ml-8'>
  //           <div className='font-gilroy text-base md:text-3xl font-medium flex items-center gap-4'>
  //             <div className='flex gap-1 sm:gap-2'>
  //               {
  //                 TokenData?.PoolData.map((token, index) => (
  //                   <div key={index}>
  //                     <div className='bg-[#3D3F47] p-1 rounded-lg'>
  //                       <img src={token.ImagePath} alt="" className='w-6 h-6 md:w-10 md:h-10' />
  //                     </div>
  //                   </div>
  //                 ))
  //               }
  //             </div>
  //             <div className='flex items-center'>
  //               <span  >{TokenData?.PoolData[0].ShortForm}</span>
  //               {
  //                 TokenData?.PoolData.slice(1).map((token, index) => (
  //                   <div key={index} className=''>
  //                     <span className='mx-0.5'>/</span>
  //                     {token.ShortForm}
  //                   </div>
  //                 ))
  //               }
  //               <span className='mx-1'>:  :</span>

  //               <span>{TokenData?.PoolData[0].weights}</span>
  //               {
  //                 TokenData?.PoolData.slice(1).map((token, index) => (
  //                   <div key={index} className=''>
  //                     <span className='mx-0.5'>/</span>
  //                     {token.weights}
  //                   </div>
  //                 ))
  //               }
  //             </div>
  //           </div>
  //         </div>
  //         <div className='flex flex-col lg:flex-row w-full gap-11 mx-auto  mt-7'>
  //           <div className=' lg:w-[59%] p-4 text-[#4b4b4b] bg-[#000711] '>
  //             {/* pool info chart here in this div */}
  //             <div>
  //               <div className='flex justify-between'>
  //                 <p className=' sm:text-3xl text-white font-semibold'>$125,625,175</p>
  //                 <div className='flex flex-col gap-y-2'>
  //                   <div className='flex justify-around gap-x-4 text-sm '>
  //                     <div>
  //                       <p>Volumes in Past-</p>
  //                       <hr className='border-[#4b4b4b]' />
  //                     </div>
  //                     <select name="" id="" className='bg-[#000711] text-white p-1 border-[1px] border-[#C0D9FF66] focus:outline-none rounded-md'>
  //                       <option value="volume">Volume</option>
  //                       <option value="24hr">24hr Vol</option>
  //                     </select>
  //                   </div>
  //                   <div className='flex gap-x-2 text-sm'>
  //                     {
  //                       selectRang.map((rang, index) => 
  //                         <div className='flex flex-col items-center' key={index} onClick={() => {
  //                           setCurrentRange(index)
  //                         }}>
  //                           <p className='cursor-pointer'>{rang}</p>
  //                           <span className={`p-[2px] w-1 bg-[#F7931A]  ${currentRang === index ? 'visible' : 'invisible'}`}></span>
  //                         </div>
  //                       )
  //                     }

  //                   </div>
  //                 </div>
  //               </div>
  //               <Echarts />
  //             </div>
  //             <div>

  //             </div>
  //           </div>

  //           <div className=' flex flex-col items-center gap-4 my-4 '>
  //             <div className='w-full sm:w-auto flex gap-4 h-20 lg:h-48 justify-center'>
  //               <PoolInfoBox Heading={'Pool Value'} Data={`$ ${TokenData?.PoolMetaData.PoolValue.toLocaleString('en-US')}`} />
  //               <PoolInfoBox Heading={'24H_Fees'} Data={`$ ${TokenData?.PoolMetaData.TwentyFourHourFees.toLocaleString('en-US')}`} />
  //             </div>
  //             <div className='w-full sm:w-auto flex gap-4 h-20 lg:h-48 justify-center'>
  //               <PoolInfoBox Heading={'24H_Pool Volume'} Data={`$ ${TokenData?.PoolMetaData.TwentyFourHourVolume.toLocaleString('en-US')}`} />
  //               <PoolInfoBox Heading={'APR'} Data={`${TokenData?.PoolMetaData.APRstart}% - ${TokenData?.PoolMetaData.APRend}%`} />
  //             </div>
  //           </div>
  //         </div>

  //         <div className='gap-2 pt-9 mx-10 font-gilroy flex items-center'>
  //           <span className='text-base leading-5 font-bold opacity-75 tracking-wide'>My Pool Balance:</span>
  //           <span className='mx-3 text-2xl font-normal leading-6'>${TokenData?.PoolMetaData.PersonalPoolBalance.toLocaleString('en-US')}</span>
  //         </div>


  //         {/* <div className='flex gap-3 md:gap-6 my-4 mx-3 md:mx-10'>
  //           <div>
  //             <GradientButton CustomCss={`text-xs md:text-base lg:text-base  lg:w-[150px] py-2`}>
  //               Swap Tokens
  //             </GradientButton>
  //           </div>
  //           <div>
  //             <GradientButton CustomCss={`text-xs md:text-base lg:text-base  lg:w-[150px] py-2`}>
  //               Add Liquidity
  //             </GradientButton>
  //           </div>
  //           <div>
  //             <GradientButton CustomCss={`text-xs md:text-base lg:text-base  lg:w-[150px] py-2`}>
  //               Withdraw
  //             </GradientButton>
  //           </div>
  //         </div> */}

  //         {/* <div className='font-gilroy font-medium text-base md:text-xl lg:text-2xl flex gap-3 md:gap-16 lg:gap-32 mx-4 lg:mx-10 mt-6'>
  //           {Heading.map((heading, index) => (
  //             <div className='flex flex-col justify-center text-center items-center gap-2 cursor-pointer' key={index}
  //               onClick={() => {
  //                 setCurrIndex(index)
  //               }}>
  //               <h1>{heading}</h1>
  //               <span className={`p-[1px]  bg-[#F7931A] w-full ${currIndex === index ? 'visible' : 'invisible'}`}></span>
  //             </div>
  //           ))}
  //         </div> */}


  //         {/* <div >
  //           {currIndex === 0 && <PoolCompositions TableData={TokenData?.PoolData} />}
  //           {currIndex === 1 && <Swapping id={Number(id)} />}
  //           {currIndex === 2 && <LiquidityOverview id={id} />}
  //         </div> */}



  //       </div>

  //     </div>
  //   </div>
  // )
  
}

export default AddLiquidity 
