import React, { useEffect, useState, useRef, useCallback } from 'react';
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
import { idlFactory as tokenIdl } from '../../../../declarations/ckbtc_ledger';

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

  useEffect(() => {
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
    let decimals = null;
    let fee = null;
    let amount = null;
    let balance = null;
    const metaData = await tokenActor.icrc1_metadata();

    for (const item of metaData) {
      if (item[0] === 'icrc1:decimals') {
        decimals = Number(item[1].Nat);
      } else if (item[0] === 'icrc1:fee') {
        fee = Number(item[1].Nat);
      }
    }

    amount = await parseInt(Number(sendAmount) * Math.pow(10, decimals));
    balance = await getBalance(canisterId);

   if (balance >= amount + fee){
    const transaction = {
      idl: tokenIdl,
      canisterId: canisterId,
      methodName: 'icrc2_approve',
      args: [
        {
          amount: BigInt(amount + fee),
          from_subaccount: [],
          spender: {
            owner: Principal.fromText(backendCanisterID),
            subaccount: [],
          },
          fee: [],
          memo: [],
          created_at_time: [],
          expected_allowance: [],
          expires_at: [],
        },
      ],
      onSuccess: async (res) => {
        console.log(`Approval successful for token: ${canisterId}`, res);
      },
      onFail: (res) => {
        console.error(`Approval failed for token: ${canisterId}`, res);
      },
    };
    return transaction;
   }else{
    {
      console.error("Insufficient balance:", balance, "required:", amount + fee);
      return { success: false, error: "Insufficient balance" };
    }
   }
 

  };

  
  

  // handleCreatePoolClick Function
  const handleCreatePoolClick = async (backendCanisterID) => {
    try {
      if (!Tokens || Tokens.length === 0) {
        console.error('Tokens array is empty or undefined');
        return { success: false, error: 'No tokens to process' };
      }

      const approvalTransactions = await Promise.all(
        Tokens.map(async (tokenData) => {
          if (!tokenData.CanisterId || !tokenData.Amount) {
            const errorMsg = `Invalid token data: ${JSON.stringify(tokenData)}`;
            console.error(errorMsg);
            throw new Error(errorMsg);
          }
          const tokenActor = await createTokenActor(tokenData.CanisterId);
          const transaction = await transferApprove(
            tokenData.Amount,
            tokenData.CanisterId,
            backendCanisterID,
            tokenActor
          );
          return transaction;
        })
      );
          console.log("approvalTransactions", approvalTransactions)
      // Execute batch transactions
      window["approvalTransaction"] = approvalTransactions
      const result = await window.ic.plug.batchTransactions(approvalTransactions);


      console.log('All tokens approved successfully');
      toast.success("Approve successfully ")
      return { success: true, data: result };
    } catch (error) {
      console.error('Error in handleCreatePoolClick:', error);
      toast.error('Token approval failed. Please try again.');
      return { success: false, error: error.message };
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
      <div className='z-50 w-max m-auto flex flex-col gap-4 p-3 sm:p-6 bg-gradient-to-b from-[#3E434B] to-[#02060D] border mx-auto rounded-lg'>
        <div className='w-[78%] sm:w-[74%] place-self-end  flex justify-between'>
          <span className='font-fahkwang font-light text-base sm:text-3xl '>Set Initial Liquidity</span>
          <div className='sm:hidden block'>
            <Bolt size={22} className='cursor-pointer' onClick={() => { console.log("settings open") }} />
          </div>
          <div className='sm:block hidden'>
            <Bolt size={30} className='cursor-pointer' onClick={() => { console.log("settings open") }} />
          </div>
        </div>

        <div className='flex justify-between gap-12 items-center font-cabin'>
          <div className='flex flex-col'>
            <div>
              <input
                className="font-normal leading-5 text-xl sm:text-3xl py-1 inline-block bg-transparent border-none outline-none"
                type="number"
                value={initialTokenAmount}
                ref={initialTokenRef}
                onChange={(e) => handleInput(e, 0)}
              />
            </div>
            <span className='text-sm sm:text-base font-normal'>
              Balance: {initialTokenBalance}
            </span>
          </div>
          <div className='flex flex-col justify-center'>
            <div className='flex gap-3 items-center'>
              <BlueGradientButton customCss={'disabled px-2 py-2  normal-cursor'}>
                <img src={InitialToken.ImagePath} alt="" className=' h-3 w-3 sm:h-6 sm:w-6 transform scale-150' />
              </BlueGradientButton>
              <span className='text-base sm:text-2xl font-normal'>
                {InitialToken.ShortForm}
              </span>
              <span className='bg-[#3E434B] py-1 rounded-lg px-2 sm:px-3'>
                {InitialToken.weights} %
              </span>
            </div>
            <span className='text-center font-normal leading-5 text-sm sm:text-base'>
              $ {InitialToken.currencyAmount || 0}
            </span>
          </div>
        </div>

        <div>
          {RestTokens.map((token, index) => {
            const balance = restTokensBalances[index];

            return (
              <div key={index}>
                <div className='border-t-[1px] opacity-50 item-center my-6'></div>
                <div className='flex justify-between items-center font-cabin'>
                  <div className='flex flex-col'>
                    <div>
                      <input
                        className="font-normal leading-5 text-xl sm:text-3xl py-1 inline-block outline-none bg-transparent"
                        type="number"
                        value={restTokensAmount[index]}
                        ref={(el) => (restTokensRefs.current[index] = el)}
                        onChange={(e) => handleInput(e, index + 1)}
                      />
                    </div>
                    <span className='text-sm sm:text-base font-normal'>
                      Balance: {balance}
                    </span>
                  </div>
                  <div className='flex flex-col justify-center'>
                    <div className='flex gap-3 items-center'>
                      <BlueGradientButton customCss={'disabled px-2 py-2  normal-cursor'}>
                        <img src={token.ImagePath} alt="" className='h-3 w-3 sm:h-6 sm:w-6 transform scale-150' />
                      </BlueGradientButton>
                      <span className='text-sm sm:text-2xl font-normal'>
                        {token.ShortForm}
                      </span>
                      <span className='bg-[#3E434B] py-1 rounded-lg px-2 sm:px-3'>
                        {token.weights} %
                      </span>
                    </div>
                    <span className='text-center font-normal leading-5 text-sm sm:text-base'>
                      $ {token.currencyAmount || 0}

                    </span>
                  </div>
                </div>
              </div>
            );
          })}
        </div>

        {Confirmation && <FinalizePool handleCreatePoolClick={handleCreatePoolClick} />}
        <div
          className={`font-cabin text-base font-medium`}
          onClick={() => {
            if(!isAuthenticated){
              toast.warn('Please login first')
            }else if (!ButtonActive) {
              toast.warn('Please select all the coins')
            } else if (!AmountSelectCheck) {
              toast.warn('You do not have enough tokens.')
            } else {
              console.log("dispatched called");
              dispatch(toggleConfirm({
                value: true,
                page: "Initial Page"
              }));
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

