import React, { useEffect, useState, useRef, useCallback, useMemo } from 'react';
import { Bolt } from 'lucide-react';
import BlueGradientButton from '../../buttons/BlueGradientButton';
import FinalizePool from './FinalizePool';
import GradientButton from '../../buttons/GradientButton';
import { showAlert, hideAlert } from '../../reducer/Alert';
import { useDispatch, useSelector } from 'react-redux';
import { UpdateAmount, toggleConfirm } from '../../reducer/PoolCreation';
import { useAuth } from '../../components/utils/useAuthClient';
import { Principal } from '@dfinity/principal';
import { searchCoinGeckoById } from '../../components/utils/fetchCoinGeckoData';
import { toast } from 'react-toastify';
import { idlFactory as tokenIdl } from '../../../../declarations/ckbtc/index';
import { IOSSwitch } from '../../buttons/SwitchButton';

const InitialLiquidity = () => {
  const dispatch = useDispatch();
  const { createTokenActor, principal, getBalance } = useAuth();
  const [tokenActor, setTokenActor] = useState(null);
  const [initialTokenBalance, setInitialTokenBalance] = useState(0);
  const [restTokensBalances, setRestTokensBalances] = useState([]);
  const [initialTokenAmount, setInitialTokenAmount] = useState(0);
  const [restTokensAmount, setRestTokensAmount] = useState([]);
  const [ButtonActive, SetButtonActive] = useState(false);
  const [AmountSelectCheck, setAmountSelectCheck] = useState(false);
  const [tokenApiDetails, setTokenApiDetails] = useState()

  const { Tokens, Confirmation } = useSelector((state) => state.pool);
  const { backendActor, isAuthenticated } = useAuth();
  const initialTokenRef = useRef(null);
  const restTokensRefs = useRef([]);
  const [optimizeEnable, setOptimizeEnable] = useState(true)
  const [pooExits, setPoolExist] = useState();
  const [poolName, setPoolName] = useState();
  // useEffect(() => {
  //     if(Tokens.length > 0){
  //         let onePercentPrice = Tokens[0].marketPrice / Tokens[0].weights; 
  //         setOnePercent(onePercentPrice)   
         
     
  //     }
  // })

  useEffect(() => {
    const concatenatedNames = Tokens.map((token) => token.ShortForm).join('');
    setPoolName(concatenatedNames);
  }, [Tokens]);
  
  
  const fetchPoolName = async (id) =>{
    const pool = await backendActor.get_specific_pool_data(id)
    // const poolDataArray = pool;
    console.log("specific pool data array", pool);
    setPoolExist(pool);
   if(!pool.Ok){
    dispatch(toggleConfirm({
      value: true,
      page: "Initial Page"
    }));
   }else{
    toast.warn('Pool already exist')
   }
   }

  const handleOptimize = () =>{
    let onePercentPrice = Tokens[0].currencyAmount / Tokens[0].weights; 
      Tokens.slice(1).forEach((token, index) => {
        let totalPrice = token.weights * onePercentPrice
         console.log("token.currencyAmount", totalPrice, token.currencyAmount )
         //  console.log(totalPrice / token.marketPrice)
         let newValue = totalPrice / token.marketPrice
         let newIndex = 1 + index
         console.log("idex", newIndex, newValue)
         dispatch(UpdateAmount({ index: newIndex, Amount: newValue }))
        // 
      })
  }

  useMemo(()=> {
    console.log("optimizeEnable", optimizeEnable, initialTokenBalance)
    if(optimizeEnable){
      handleOptimize()
    }else{
      return;
    }
  }, [optimizeEnable, initialTokenAmount, restTokensAmount])
  
  useEffect(() => {
    if (Tokens.length <= 0) {
      return;
    }
    const fetchSelectedToken = async () => {
      const fetchedSelectedToken = await Promise.all(
        Tokens?.map(async (list, index) => {
          let name = list.id.toLowerCase()
          const tokenDetail = await searchCoinGeckoById(name)

          return {
            tokenDetail
          }
        })
      )
      setTokenApiDetails(fetchedSelectedToken)
    }
    fetchSelectedToken()
  }, [])


  useEffect(() => {
    if (Tokens.length > 0) {
      setInitialTokenAmount(Tokens[0].Amount);
      setRestTokensAmount(Tokens.slice(1).map((token) => token.Amount));
    }
  }, [Tokens]);



  const fetchTokenActor = useCallback(async () => {
    if (Tokens.length > 0) {
      const actor = await createTokenActor(Tokens[0].CanisterId);
      setTokenActor(actor);
    }
  }, [Tokens, createTokenActor]);


  useEffect(() => {
    fetchTokenActor();
  }, [fetchTokenActor]);

  useEffect(() => {
    const fetchInitialTokenBalance = async () => {
      if (tokenActor && principal) {
        const balance = await tokenActor.icrc1_balance_of({ owner: principal, subaccount: [] });
        setInitialTokenBalance(parseFloat(Number(balance) / 100000000));
      }
    };
    fetchInitialTokenBalance();
  }, [tokenActor, principal]);

  const fetchRestTokensBalances = useCallback(async () => {
    if (Tokens.length > 1) {
      const balances = await Promise.all(
        Tokens.slice(1).map(async (token) => {
          const balance = await getBalance(token.CanisterId)

          return parseFloat(Number(balance) / 100000000);
        })
      );
      setRestTokensBalances(balances);
    }
  }, [Tokens, createTokenActor, principal]);

  useMemo(() => {
    fetchRestTokensBalances();
  }, [fetchRestTokensBalances]);


  const handleInput = (event, index) => {
    const newValue = parseFloat(event.target.value);
    if (index === 0 && newValue !== 0) {
      setInitialTokenAmount(newValue);
    } else {
      const newAmounts = [...restTokensAmount];
      newAmounts[index - 1] = newValue;
      setRestTokensAmount(newAmounts);
    }
    dispatch(UpdateAmount({ index, Amount: newValue }));
  };

  const HandleSelectCheck = useCallback(() => {
    const allTokensSelected = Tokens.every((token) => token.Selected);
    SetButtonActive(allTokensSelected);
    const amountsValid =
      initialTokenAmount <= initialTokenBalance &&
      restTokensAmount.every((amount, index) => amount <= restTokensBalances[index]);
    setAmountSelectCheck(amountsValid);
  }, [Tokens, initialTokenAmount, initialTokenBalance, restTokensAmount, restTokensBalances]);

  useEffect(() => {
    HandleSelectCheck();
  }, [Tokens, HandleSelectCheck]);

  if (Tokens.length === 0) {
    return null;
  }

  const InitialToken = Tokens[0];
  const RestTokens = Tokens.slice(1);


  // token Approval function
  const transferApprove = async (sendAmount, canisterId, backendCanisterID, tokenActor) => {
        try {
            let decimals = null;
            let fee = null;
            let amount = null;
            let balance = null;
            const metaData = await tokenActor.icrc1_metadata();
            for (const item of metaData) {
                if (item[0] === 'icrc1:decimals') {
                    decimals = Number(item[1].Nat); // Assuming decimals is stored as a Nat (BigInt)
                } else if (item[0] === 'icrc1:fee') {
                    fee = Number(item[1].Nat); // Assuming fee is stored as a Nat (BigInt)
                }
            }
            amount = await parseInt(Number(sendAmount) * Math.pow(10, decimals));
            balance = await getBalance(canisterId);
            const extra = 10000;


            console.log("init metaData", metaData);
            console.log("init decimals", decimals);
            console.log("init fee", fee);
            console.log("init amount", amount);
            console.log("init balance", balance);

            if (balance >= amount + fee) {
                const transaction = {
                    amount: BigInt(amount + fee + extra),  // Approving amount (including fee)
                    from_subaccount: [],  // Optional subaccount
                    spender: {
                        owner: Principal.fromText(backendCanisterID),
                        subaccount: [],  // Optional subaccount for the spender
                    },
                    fee: [],  // Fee is optional, applied during the transfer
                    memo: [],  // Optional memo
                    created_at_time: [],  // Optional timestamp
                    expected_allowance: [],  // Optional expected allowance
                    expires_at: [],  // Optional expiration time
                };

                // console.log("transaction", transaction);

                const response = await tokenActor.icrc2_approve(transaction);

                if (response?.Err) {
                    console.error("Approval error:", response.Err);
                    toast.error("approve failed")
                    return { success: false, error: response.Err };
                } else {
                    console.log("Approval successful:", response);
                    toast.success("approve success")
                    return { success: true, data: response.Ok };
                }
            } else {
                console.error("Insufficient balance:", balance, "required:", amount + fee);
                return { success: false, error: "Insufficient balance" };
            }
        } catch (error) {
            toast.error("approve failed")
            console.error("Error in transferApprove:", error);
            return { success: false, error: error.message };
        }
    };

  
  

  // handleCreatePoolClick Function
  const handleCreatePoolClick = async (backendCanisterID) => {
    try {
      if (!Tokens || Tokens.length === 0) {
        console.error("Tokens array is empty or undefined");
        return { success: false, error: "No tokens to process" };
      }
      const approvalResults = [];
      await Tokens.reduce((promiseChain, tokenData, index) => {
        return promiseChain.then(async () => {
          console.log(`Processing token ${index + 1} / ${Tokens.length}:`, tokenData);
  
          if (!tokenData.CanisterId || !tokenData.Amount) {
            const errorMsg = `Invalid token data at index ${index}: ${JSON.stringify(tokenData)}`;
            console.error(errorMsg);
            approvalResults.push({ success: false, error: errorMsg, token: tokenData });
            // Decide whether to continue or reject the chain
            // return Promise.resolve(); // To continue
            return Promise.reject({ success: false, error: errorMsg, token: tokenData }); // To exit on error
          }
          const decimals = tokenData.metaData.decimals;
          const tokenActors = await createTokenActor(tokenData.CanisterId);
          const approvalResult = await transferApprove(
            tokenData.Amount,
            tokenData.CanisterId,
            backendCanisterID,
            tokenActors
          );
  
          if (!approvalResult.success) {
            console.error(`Approval failed for token: ${tokenData.CanisterId}`, approvalResult.error);
  
            approvalResults.push({ success: false, error: approvalResult.error, token: tokenData });
            // Decide whether to continue or reject the chain
            // return Promise.resolve(); // To continue
            return Promise.reject({ success: false, error: approvalResult.error, token: tokenData }); // To exit on error
          } else {
            console.log(`Approval successful for token: ${tokenData.CanisterId}`);
            approvalResults.push({ success: true, data: approvalResult.data, token: tokenData });
          }
        });
      }, Promise.resolve());
  
      const failedApprovals = approvalResults.filter(result => !result.success);
  
      if (failedApprovals.length > 0) {
        console.error("Some token approvals failed:", failedApprovals);
        return { success: false, error: "Some token approvals failed", details: failedApprovals };
      }
  
      console.log("All tokens approved successfully");
      return { success: true };
    } catch (error) {
      console.error("Error in handleCreatePoolClick:", error);
      // The error object might be our custom error from reject
      return error.success !== undefined ? error : { success: false, error: error.message };
    }
  };
  
  


  return (
    <div className=''>
      <div className='w-full'>
        <div className={`flex gap-6 pb-6 w-[70%] md:w-[60%] justify-between items-center m-auto  lg:hidden`}>
          <div className={`py-2 px-4 rounded-full bg-[#F7931A]`}>3</div>
          <p className="text-lg"></p>
          <hr className="border-2 w-3/4 pr-6" />
        </div>
      </div>
      <div className=' w-max m-auto flex flex-col gap-4 p-3 sm:p-6 relative space-y-2'>
        <div className='flex gap-2 items-center justify-end w-full'>
          <p className='font-gilroy text-sm'>Auto optimize liquidity</p>
          <IOSSwitch sx={{ m: 1 }} defaultChecked  onClick={()=> setOptimizeEnable((prev)=> !prev)}/>
        </div>
        <div className='flex justify-between gap-12 items-center font-gilroy backdrop-blur-[32px] 
        md:px-6 px-3 md:py-8 py-4 rounded-xl border border-white'>
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
            ${InitialToken.currencyAmount?.toLocaleString() || 0}
            </span>
          </div>
          <div className='flex flex-col justify-center'>
            <div className='flex gap-3 items-center'>
              <img src={InitialToken.ImagePath} alt="" className=' h-3 aspect-square sm:h-4 transform scale-150 rounded-full' />
              <span className='text-base sm:text-2xl font-normal'>
                {InitialToken.ShortForm.toUpperCase()}
              </span>
              <span className='text-sm sm:text-2xl font-normal'>•</span>
              <span className='py-1 px-2 sm:px-3'>
                {InitialToken.weights} %
              </span>
            </div>
            <span className='inline-flex justify-center gap-2 w-full text-center font-normal leading-5 text-sm sm:text-base'>
            <p className={`${initialTokenAmount > initialTokenBalance ? "text-red-500" : ""}`}>{initialTokenBalance?.toLocaleString()} {InitialToken.ShortForm.toUpperCase()}</p>
              <p className='text-white bg-gray-600 rounded-md px-2 h-fit text-[12px]'>Max</p>
            </span>
          </div>
        </div>

        <div className='flex flex-col gap-4'>
          { RestTokens && RestTokens.map((token, index) => {
            const balance = restTokensBalances[index];

            return (
              <div key={index}>
                <div className='flex justify-between items-center font-gilroy backdrop-blur-[32px] 
        md:px-6 px-3 md:py-8 py-4 rounded-xl border border-white'>
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
                    ${token.currencyAmount? token.currencyAmount.toLocaleString() : "0"}
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
                    {balance !== undefined ? balance.toLocaleString() : "0"} {token.ShortForm.toUpperCase()}
                      <p className='text-white bg-gray-600 rounded-md px-2 h-fit text-[12px]'>Max</p>
                    </span>
                  </div>
                </div>
              </div>
            );
          })}
        </div>
        {( pooExits && Confirmation) && <FinalizePool handleCreatePoolClick={handleCreatePoolClick} />}
        <div
          className={`font-gilroy text-base font-medium`}
          onClick={() => {
            if(!isAuthenticated){
              toast.warn('Please login first')
            }else if (!ButtonActive) {
              toast.warn('Please select all the coins')
            } else if (!AmountSelectCheck) {
              toast.warn('You do not have enough tokens.')
            } else {
              fetchPoolName(poolName)
              console.log("dispatched finished");
            }
          }}
        >
        
          <GradientButton CustomCss={`my-2 sm:my-4 w-full md:w-full ${ButtonActive ? 'opacity-100 cursor-pointer' : 'opacity-50 cursor-default'}`}>
            Analyse Pair
          </GradientButton>
        </div>
      </div>
    </div>
  );
};

export default InitialLiquidity;
