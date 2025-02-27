import React, { useState, useEffect, useCallback, useMemo } from 'react'
import { useParams } from 'react-router-dom';
import GradientButton from '../../buttons/GradientButton'
import { IOSSwitch } from '../../buttons/SwitchButton';
import { convertTokenEquivalentUSD } from '../../utils';
import { useAuth } from '../utils/useAuthClient';

const AddLiquidity = () => {
  const { id } = useParams();
  const [tokens, setTokens] = useState([]);
  const [restTokens, setRestTokens] = useState([]);
  const [token1, setToken1] = useState(null);
  const [poolData, setPoolData] = useState([]);
  const [swapFee, setSwapFee] = useState(0);
  const { backendActor, principal, createTokenActor, getBalance } = useAuth();
  const [loading, setLoading] = useState(false);
  const [retry, setRetry] = useState({getPoolData : false, initToken : false, restToken : false})

  const initToken = useCallback(async () => {
    const initialToken = tokens[0]
    let data = {}
    console.log("II : ", initialToken)
    try{
      const response = await backendActor.get_decimals(initialToken?.ledger_canister_id);
      let decimals = 0;
      if(response?.Ok){
        decimals = parseInt(response.Ok);
      }
      if(initialToken && decimals){
        const {weight, token_name, image,  ledger_canister_id} = initialToken
        console.log("Ledger : ", ledger_canister_id.toText())
        data = await getBalance(ledger_canister_id.toText()).then(balance=>{
          console.log("Balance : ", balance, ledger_canister_id.toText())
          return {
            weights: weight.toString(),
            currencyAmount: 0,
            LongForm: "",
            ShortForm: token_name.toUpperCase(),
            ImagePath: image,
            decimals,
            balance: parseFloat(balance) / Math.pow(10, decimals),
            canisterId: ledger_canister_id
          }
      });
      }
      if(initialToken?.token_name){
        data.currencyAmount = await convertTokenEquivalentUSD(initialToken?.token_name)
      }
    }catch(err){
      console.error(err)
    }finally{
      setToken1(data)
    }

  }, [id, principal, tokens, getBalance]);

  const initRestToken = useCallback(async () => {
    const splittedTokenArr = tokens.slice(1);
  
    try {
      // Step 1: Fetch metadata for all tokens in parallel
      const metadataResults = await Promise.all(splittedTokenArr.map(async (token) => {
        const canisterId = token?.ledger_canister_id
        try{
          const response = await backendActor.get_decimals(canisterId);
          console.log(response, "Meta")
          if(response?.Ok){
            const decimals = parseInt(response.Ok);
            return {
              token,
              decimals,
              canisterId
            };
          }
        }catch(err){
          console.error(err)
          return {
            token,
            decimals : null,
            canisterId
          };
        } finally{
          setLoading(false);
        }
      }));
  
      // Step 2: Process tokens and fetch USD equivalents in parallel
      const restTT = await Promise.all(metadataResults.map(async ({ token, decimals, canisterId }) => {
        if (!decimals) return null; // Skip if decimals are missing
        const data = await getBalance(canisterId.toText()).then(balance=>{
        return {
          ImagePath: token.image,
          ShortForm: token.token_name.toUpperCase(),
          weights: parseFloat(token.weight),
          balance: parseFloat(balance) / Math.pow(10, decimals), // TODO : Use fetched decimals
          CanisterId: canisterId,
          currencyAmount: 0 ,// Initialize for now,
          decimals
        };
      })
  
        // Fetch USD conversion
        data.currencyAmount = await convertTokenEquivalentUSD(token?.token_name);
        return data;
      }));

      console.log(restTT)
  
      setRestTokens(restTT); // Remove any null values
    } catch (err) {
      console.error(err);
    }
  }, [id, principal, tokens]);

  const getPoolData = useCallback(async () => {
    try {
      setLoading(true);
      const data = await backendActor.get_specific_pool_data(id);
      if (data?.Ok) {
        const pool_datas = data.Ok;
        setPoolData(pool_datas);
        setTokens(pool_datas[0].pool_data);
        setSwapFee(pool_datas[0].swap_fee);
      } else {
        throw new Error(data.Err);
      }
    } catch (err) {
      console.error("Error fetching pool data", err);
      setTokens([]);
      if(err?.code === 3000)
      setRetry((prev)=>({...prev,getPoolData : true}))
    }
  }, [id]);

  useEffect(() => {
    getPoolData();
  }, [id, principal, retry.getPoolData]);

  useEffect(() => {
    if (tokens.length > 0) {
      initToken();
      initRestToken();
    }
  }, [tokens, initToken, initRestToken]);

  const [optimizeEnable, setOptimizeEnable] = React.useState(true);
  const [initialTokenAmount, setInitialTokenAmount] = React.useState(0);
  const [equivalentUSD, setEquivalentUSD] = React.useState(0);
  const initialTokenRef = React.useRef(null);
  const [restTokensAmount,setRestTokenAmount] = useState([]);
  const restTokensRefs = React.useRef([]);

  const calculateTotal = useCallback(()=>{
    const total = restTokensAmount.reduce((acc,amount)=>{
      return acc + parseFloat(amount)
    },initialTokenAmount)
    return total + calculatePoolLocked() + calculatePoolShare() + parseFloat(swapFee)
  },[initialTokenAmount,restTokensAmount])


  const calculatePoolShare = useCallback(()=>{
    return 0.001
  },[])

  const calculatePoolLocked = useCallback(()=>{
    return 0
  },[])

  const calculateResult = useCallback((type)=>{
    let ans;
    switch(type){
      case "total":
        ans = "$" + calculateTotal()
        break;
      case "pool_share":
        ans = calculatePoolShare()
        break;
      case "gas_fee":
        ans = swapFee.toLocaleString()
        break;
      case "total_pool_value_locked":
        ans = calculatePoolLocked()
        break;
      default:
        break;
    }
    return ans
  }, [calculateTotal, calculatePoolShare, swapFee])

  const Result = useMemo(()=>({
    heading : 'Total',
    headingData : calculateResult("total"),
    data : [
      {
        title : 'Total Pool value locked',
        value : calculateResult("total_pool_value_locked")
      },
      {
        title : 'Your pool share',
        value : calculateResult("pool_share"),
      },
      {
        title : 'Gas fee',
        value : calculateResult("gas_fee")
      }
    ]
  }), [tokens,initialTokenAmount,restTokensAmount,swapFee])

  console.log("Init : \n",token1,"\nRest :",restTokens)

  const runApproval = useCallback(async(approveParams)=>{
    try{
      if(typeof approveParams === "undefined" || approveParams.length === 0) throw new Error("Approval Params Type Error")
      approveParams.forEach(async(param)=>{
        createTokenActor(param.spender.toText()).then(actor=>{
          console.log("Actor : ", actor)
          actor.approve(param)
        })
        .catch(err=>console.error(err))
      })
    }catch(err){
      console.error(err)
    }finally{
      console.log("Approval Done")
    }
  }, [createTokenActor])

  const addLiquidity = useCallback(async()=>{
    let approveParams=[]
    const pool_data = poolData.map((pool)=>{
      const params = pool.pool_data
      const param = params.map((token,index)=>{
        const amount = index === 0 ? initialTokenAmount : parseInt(restTokensAmount[index - 1])
        const decimal = index === 0 ? token1?.decimals : restTokens[index - 1]?.decimals // TODO : Use fetched decimals
        const balance = index  === 0 ? token1?.balance : restTokens[index - 1]?.balance
        console.log("TYPE FOF AMOUNT : ", typeof amount, amount)
        let approveEntry = {
          fee : 30,
          memo : [],
          amount : BigInt(amount) * BigInt(10 ** decimal),
          spender : token.ledger_canister_id
        }
        approveParams.push(approveEntry)
        return {
          value : BigInt(amount) * BigInt(10 ** decimal),
          weight : parseFloat(token.weight),
          token_name : token.token_name,
          ledger_canister_id : token.ledger_canister_id,
          image : token.image,
          balance : parseFloat(balance) * Math.pow(10, decimal)
        }
      })
      console.log("Param : ", param)
      return param
    })
    console.log(pool_data)
    try{
      runApproval(approveParams)
      const response = await backendActor.create_pools({pool_data : pool_data[0], swap_fee : parseFloat(swapFee) || 0})
      console.log(response)
      if(response?.Ok){
        console.log("Success")
      }else{
        throw new Error(JSON.stringify(response.Err))
      }
    }catch(err){
      console.error(err)
    }
  },[poolData,initialTokenAmount,restTokensAmount,swapFee, runApproval])

  const handleInput = (e) => {
    const value = parseFloat(e.target.value) || 0;
    setInitialTokenAmount(value);
  };

  const ButtonActive = true; // Static value
  const isAuthenticated = true; // Static value
  const AmountSelectCheck = true; // Static value

  // Function to calculate equivalent rest token amounts
  const calculateEquivalentAmounts = useCallback(() => {
    if (!token1?.currencyAmount || !token1?.weights){ 
      console.error("Missing required data for first token", token1);
      return
    };
    const token1USD = token1.currencyAmount * initialTokenAmount;
    console.log(token1USD,"Token1USD")
    if(token1USD <= 0) return;
    const totalPoolValue = token1USD / (parseInt(token1.weights) / 100);
    console.log("Total pool value:", totalPoolValue);
    const equivalentAmounts = restTokens.map((token, index) => {
      const tokenTargetUSDValue = totalPoolValue * (parseInt(token.weights) / 100);
      console.log("Token Target", tokenTargetUSDValue)
      const requiredTokenAmount = tokenTargetUSDValue / token.currencyAmount;
      console.log("Required Token Amount", requiredTokenAmount)
      const roundedAmount = Number(requiredTokenAmount.toFixed(8));
      return roundedAmount;
      })

    console.log("Equivalent Amounts : ", equivalentAmounts);
    setRestTokenAmount(equivalentAmounts);
  }, [token1, backendActor,restTokens, initialTokenAmount]);

  // Call the function after fetching the pool data
  useEffect(() => {
    if (tokens.length > 0) {
      calculateEquivalentAmounts();
    }
  }, [restTokens, calculateEquivalentAmounts,token1?.currencyAmount,initialTokenAmount,optimizeEnable]);

  const handleRestTokenInput = (e, index) => {
    const value = parseFloat(e.target.value) || 0;
    const newAmounts = restTokensAmount.map((amount, i) => {
      if (i === index) return value;
      return amount;
    });
    setRestTokenAmount(newAmounts);
  }

  if (loading) {
    return <div>Loading...</div>;
  }

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
                className={`${initialTokenAmount > token1?.balance ? "text-red-500" : ""} font-normal leading-5 text-xl sm:text-3xl py-1 inline-block bg-transparent border-none outline-none`}
                type="number"
                min='0'
                value={isNaN(initialTokenAmount) ? "" : initialTokenAmount}
                ref={initialTokenRef}
                onChange={(e) => handleInput(e)}
              />
            </div>
            <span className='text-sm sm:text-base font-normal'>
              ${(token1?.currencyAmount * initialTokenAmount) || 0}
            </span>
          </div>
          <div className='flex flex-col justify-center'>
            <div className='flex gap-3 items-center'>
              <img src={token1?.ImagePath} alt="" className='h-3 aspect-square sm:h-4 transform scale-150 rounded-full' />
              <span className='text-base sm:text-2xl font-normal'>
                {token1?.ShortForm}
              </span>
              <span className='text-sm sm:text-2xl font-normal'>•</span>
              <span className='py-1 px-2 sm:px-3'>
                {token1?.weights} %
              </span>
            </div>
            <span className='inline-flex justify-center gap-2 w-full text-center font-normal leading-5 text-sm sm:text-base'>
              <p className={`${initialTokenAmount > token1?.balance ? "text-red-500" : ""}`}>{token1?.balance} {token1?.ShortForm}</p>
              <p className='text-white bg-gray-600 rounded-md px-2 h-fit text-[12px]'>Max</p>
            </span>
          </div>
        </div>

        <div className='flex flex-col gap-4'>
          {restTokens.map((token, index) => {
            const balance = token?.balance;

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
                        value={restTokensAmount[index]}
                        placeholder="0"
                        ref={(el) => (restTokensRefs.current[index] = el)}
                        onChange={(e) => handleRestTokenInput(e, index)}
                        disabled={optimizeEnable}
                      />
                    </div>
                    <span className='text-sm sm:text-base font-normal'>
                      ${parseInt(restTokensAmount[index]) * token?.currencyAmount || "0"}
                    </span>
                  </div>
                  <div className='flex flex-col justify-center'>
                    <div className='flex gap-3 items-center'>
                      <img src={token?.ImagePath} alt="" className='h-3 aspect-square sm:h-4 transform scale-150 rounded-full' />
                      <span className='text-sm sm:text-2xl font-normal'>
                        {token?.ShortForm.toUpperCase()}
                      </span>
                      <span className='text-sm sm:text-2xl font-normal'>•</span>
                      <span className='py-1 px-2 sm:px-3'>
                        {token?.weights} %
                      </span>
                    </div>
                    <span className='inline-flex justify-center gap-2 text-center font-normal leading-5 text-sm sm:text-base'>
                      {balance.toLocaleString()} {token?.ShortForm.toUpperCase()}
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
              addLiquidity();
            }
          }}
        >
          <GradientButton CustomCss={`my-2 sm:my-4 w-full md:w-full ${ButtonActive ? 'opacity-100 cursor-pointer' : 'opacity-50 cursor-default'}`}>
            {initialTokenAmount == 0 ? 'Add Token Amount' : 'Add Liquidity'}
          </GradientButton>
        </div>
        <table className='w-full font-gilroy'>
          <thead className='text-xl font-semibold'>
            <tr className='flex justify-between w-full'>
              <th>{Result.heading}</th>
              <th>{Result.headingData}</th>
            </tr>
          </thead>
          <tbody className='text-base'>
            {Result.data.map((data, index) => (
              <tr key={index}>
                <td>{data.title}</td>
                {data.title === 'Gas fee' ? (
                  <td>{`${data.value} ${token1?.ShortForm} ( $${equivalentUSD} )`}</td>
                ) : (
                  <td>{data.value.toLocaleString()}</td>
                )}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

export default AddLiquidity;
