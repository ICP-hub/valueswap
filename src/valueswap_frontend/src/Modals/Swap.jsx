import React, { useState, useEffect } from 'react'
import { toast } from 'react-toastify'
import { useAuth } from '../components/utils/useAuthClient'
import { Principal } from '@dfinity/principal'
import {
  Settings as SettingsIcon,
  Cached as CachedIcon
} from '@mui/icons-material'
import BorderGradientButton from '../buttons/BorderGradientButton'
import GradientButton from '../buttons/GradientButton'
import SearchToken from './SearchToken'
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown'
import { SwapModalData } from '../TextData'
import SwapSetting from './SwapSetting'
const Swap = () => {
  const { backendActor, getBalance, createTokenActor, isAuthenticated } =
    useAuth()

  // States
  const [payCoin, setPayCoin] = useState(null)
  const [receiveCoin, setReceiveCoin] = useState(null)
  const [coinAmount, setCoinAmount] = useState(0)
  const [payCoinBalance, setPayCoinBalance] = useState(0)
  const [receiveCoinBalance, setReceiveCoinBalance] = useState(0)
  const [approvalSuccess, setApprovalSuccess] = useState(false)
  const [swapSuccess, setSwapSuccess] = useState(false)
  const [poolNotFound, setPoolNotFound] = useState(false)
  const [ClickedSwap, setClickSwap] = useState(false)
  const [searchToken1, setSearchToken1] = useState(false)
  const [searchToken2, setSearchToken2] = useState(false)
  const [recieveValue, setReciveValue] = useState(0)
  const [balance, setBalance] = useState(0)
  const [settings, setSettings] = useState(false)
  const [Id, setId] = useState(0)
  // Fetch balances whenever payCoin or receiveCoin changes
  useEffect(() => {
    if (payCoin) {
      getBalance(payCoin.CanisterId)
        .then(balance => {
          setPayCoinBalance(Number(balance) / 100000000)
        })
        .catch(err => console.log(err))
    }
    if (receiveCoin) {
      getBalance(receiveCoin?.CanisterId)
        .then(balance => {
          setReceiveCoinBalance(Number(balance) / 100000000)
        })
        .catch(err => console.log(err))
    }
  }, [payCoin, receiveCoin])

  // Helper to fetch balance

  useEffect(() => {
    const getSwapValue = async () => {
      if (coinAmount) {
        const amount = parseFloat(coinAmount)

        const swapValue = await backendActor.pre_compute_swap({
          token1_name: payCoin.ShortForm,
          token_amount: amount,
          token2_name: receiveCoin.ShortForm,
          ledger_canister_id1: Principal.fromText(receiveCoin.CanisterId),
          ledger_canister_id2: Principal.fromText(receiveCoin.CanisterId)
        })
        console.log('swapValue', swapValue.length)
        if (swapValue.length == 2) {
          setPoolNotFound(true)
        }
        setReciveValue(swapValue[1])
      } else {
        console.log('no coin Amount enter')
        setPoolNotFound(false)
      }
      if (payCoin) {
        getBalance(payCoin.CanisterId)
          .then(balance => {
            setPayCoinBalance(Number(balance) / 100000000)
          })
          .catch(err => console.log(err))
        // console.log("Balance", payCoinBalance);
      }
    }
    getSwapValue()
  }, [payCoin, getBalance, coinAmount, receiveCoin])

  useEffect(() => {
    if (receiveCoin) {
      getBalance(receiveCoin?.CanisterId)
        .then(balance => {
          setReceiveCoinBalance(Number(balance) / 100000000)
        })
        .catch(err => console.log(err))
    }
  }, [receiveCoin, getBalance])

  const ClickChangeHandler = () => {
    let temp = receiveCoin
    setReceiveCoin(payCoin)
    setPayCoin(temp)
  }

  // Handle amount input change
  const handleAmountChange = e => {
    const value = e.target.value
    setCoinAmount(value >= 0 ? parseFloat(value) : 0)
  }

  // Handle token approval
  const transferApprove = async (
    sendAmount,
    canisterId,
    backendCanisterID,
    tokenActor
  ) => {
    try {
      let decimals = null
      let fee = null
      let amount = null
      let balance = null
      const metaData = await tokenActor.icrc1_metadata()
      for (const item of metaData) {
        if (item[0] === 'icrc1:decimals') {
          decimals = Number(item[1].Nat) // Assuming decimals is stored as a Nat (BigInt)
        } else if (item[0] === 'icrc1:fee') {
          fee = Number(item[1].Nat) // Assuming fee is stored as a Nat (BigInt)
        }
      }
      amount = await parseInt(Number(sendAmount) * Math.pow(10, decimals))
      balance = await getBalance(canisterId)

      console.log('init metaData', metaData)
      console.log('init decimals', decimals)
      console.log('init fee', fee)
      console.log('init amount', amount)
      console.log('init balance', balance)

      if (balance >= amount + fee) {
        const transaction = {
          amount: BigInt(amount + fee), // Approving amount (including fee)
          from_subaccount: [], // Optional subaccount
          spender: {
            owner: Principal.fromText(backendCanisterID),
            subaccount: [] // Optional subaccount for the spender
          },
          fee: [], // Fee is optional, applied during the transfer
          memo: [], // Optional memo
          created_at_time: [], // Optional timestamp
          expected_allowance: [], // Optional expected allowance
          expires_at: [] // Optional expiration time
        }
        // console.log("transaction", transaction);

        const response = await tokenActor.icrc2_approve(transaction)

        if (response?.Err) {
          console.error('Approval error:', response.Err)
          toast.error('approve failed')
          return { success: false, error: response.Err }
        } else {
          console.log('Approval successful:', response)
          toast.success('approve success')
          return { success: true, data: response.Ok }
        }
      } else {
        console.error(
          'Insufficient balance:',
          balance,
          'required:',
          amount + fee
        )
        return { success: false, error: 'Insufficient balance' }
      }
    } catch (error) {
      toast.error('approve failed')
      console.error('Error in transferApprove:', error)
      return { success: false, error: error.message }
    }
  }

  const backendCanisterID = process.env.CANISTER_ID_VALUESWAP_BACKEND
  const handleSwapApproval = async (payCoin, backendCanisterID) => {
    return new Promise((resolve, reject) => {
      if (!payCoin) {
        return resolve({ success: false, error: 'No token to process' })
      }

      if (!payCoin.CanisterId || !coinAmount) {
        const errorMsg = `Invalid token data: ${JSON.stringify(payCoin)}`
        console.log(errorMsg)
        return resolve({ success: false, error: errorMsg, token: payCoin })
      }

      createTokenActor(payCoin.CanisterId)
        .then(tokenActor => {
          // console.log("tokenActor", tokenActor);
          return transferApprove(
            coinAmount,
            payCoin.CanisterId,
            backendCanisterID,
            tokenActor
          )
        })
        .then(approvalResult => {
          if (!approvalResult.success) {
            console.error(
              `Approval failed for token: ${payCoin.CanisterId}`,
              approvalResult.error
            )
            // toast.error("approve failed")
            return resolve({
              success: false,
              error: approvalResult.error,
              token: payCoin
            })
          } else {
            console.log(`Approval successful for token: ${payCoin.CanisterId}`)
            setApprovalSuccess(true)
            return resolve({
              success: true,
              data: approvalResult.data,
              token: payCoin
            })
          }
        })
        .catch(error => {
          console.error('Error in handleSwapApproval:', error)
          return resolve({ success: false, error: error.message })
        })
    })
  }

  const swapHandler = async () => {
    console.log('Click on swap', { coinAmount, payCoin, receiveCoin })

    if (coinAmount === undefined || coinAmount === null) {
      console.error('coinAmount is undefined or null')
      return error
    }
    let amount
    try {
      amount = parseFloat(coinAmount)
    } catch (error) {
      console.error('Invalid coinAmount:', coinAmount, error)
      return error
    }
    if (!payCoin || !payCoin.ShortForm) {
      console.error('payCoin is invalid:', payCoin)
      return error
    }
    if (!receiveCoin || !receiveCoin.ShortForm) {
      console.error('receiveCoin is invalid:', receiveCoin)
      return error
    }
    if (!backendActor || !backendActor.compute_swap) {
      console.error(
        'backendActor is not available or compute_swap method is missing'
      )
      return error
    }

    try {
      console.log('Calling backendActor.compute_swap with:', {
        token_amount: amount,
        token1_name: payCoin.ShortForm,
        token2_name: receiveCoin.ShortForm,
        ledger_canister_id1: payCoin.CanisterId,
        ledger_canister_id2: receiveCoin.CanisterId
      })
      const res = await backendActor.compute_swap({
        token1_name: payCoin.ShortForm,
        token_amount: amount,
        token2_name: receiveCoin.ShortForm,
        ledger_canister_id1: Principal.fromText(payCoin.CanisterId),
        ledger_canister_id2: Principal.fromText(receiveCoin.CanisterId)
      })
      console.log('Response from compute_swap:', res)
      if (res.Ok) {
        getSwapValue()
      }
      console.log('slipage:', recieveValue)

      if (res.Ok == null) {
        console.log('Swap successful')
        // setSwapSuccess(true)
        toast.success('swap complete')
        // navigate('/valueswap/transaction-successfull');
        return res
      } else if (res && res.Err) {
        console.error('Swap failed with error:', res.Err, res)
        return res
      } else {
        console.error('Unexpected response from compute_swap:', res)
        return res
      }
    } catch (error) {
      console.error('Error while calling swap function:', error)
      return res
    }
  }

 const handleSettings = () => {
    setSettings((prev) => !prev)
 }

  return (
    <section className='h-screen relative flex flex-col items-center gap-y-12 bg-background-gradient'>
      <h1 className='text-center text-3xl mt-4 tracking-wider'>Swap</h1>
      {searchToken1 && (
        <SearchToken
          setSearchToken={setSearchToken1}
          setPayToken={setPayCoin}
          setRecToken={setReceiveCoin}
          id={Id}
        />
      )}

      <div className='relative'>
        {/* Pay Section */}
        <div className='w-[80%] sm:w-[480px] mb-2 mx-auto text-[#A3A3A3] bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-20 border  border-[#FFFFFF66] rounded-2xl p-8'>
          <div className='flex justify-between'>
            <div>Pay</div>
          </div>
          <div
            className={`flex justify-between ${
              coinAmount > payCoinBalance ? 'text-red' : ''
            }`}
          >
            <input
              type='number'
              className='bg-transparent w-1/2 outline-none hide-arrows text-4xl'
              placeholder='0'
              min='0'
              value={coinAmount}
              onChange={handleAmountChange}
            />
            <BorderGradientButton customCss={`bg-gray-700 `}>
              <div
                className='flex text-sm sm:text-base items-center gap-1 cursor-pointer'
                onClick={() => {
                  setId(1)
                  setSearchToken1(!searchToken1)
                }}
              >
                {payCoin ? (
                  <div className='flex items-center gap-x-2'>
                    <img src={payCoin.ImagePath} alt='' className='w-6 h-6' />
                    {payCoin.ShortForm}
                  </div>
                ) : (
                  'Select Token'
                )}
                <KeyboardArrowDownIcon />
              </div>
            </BorderGradientButton>
          </div>

          <div className='flex justify-between mt-2'>
            <div>${coinAmount > 0 ? coinAmount * payCoin?.marketPrice : 0}</div>
            <div>
              <button
                className='font-gilroy ml-1 sm:ml-2 text-orange-400 text-base font-normal'
                onClick={() => setCoinAmount(payCoinBalance)}
              >
                {payCoinBalance ? `${payCoinBalance} Max` : ''}
              </button>
            </div>
          </div>
        </div>

        {/* Receive Section */}
        <div className='w-[80%] sm:w-[480px] z-0 mx-auto text-[#A3A3A3] bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-20 border border-[#FFFFFF66] rounded-2xl p-8 relative'>
        
          <div className='flex justify-between'>
            <div>Receive</div>
          </div>
          <div className='flex justify-between'>
            <div className='text-4xl w-1/2 overflow-x-auto  scroll-smooth'>
              {coinAmount && payCoin && receiveCoin
                ? coinAmount * (payCoin?.marketPrice / receiveCoin?.marketPrice)
                : 0}
            </div>
            <BorderGradientButton customCss={`bg-gray-700 z-10`}>
              <div
                className='flex text-sm sm:text-base items-center gap-1 cursor-pointer'
                onClick={() => {
                  setId(2)
                  setSearchToken2(!searchToken2)
                }}
              >
                {receiveCoin ? (
                  <div className='flex items-center gap-x-2'>
                    <img
                      src={receiveCoin.ImagePath}
                      alt=''
                      className='w-6 h-6'
                    />
                    {receiveCoin.ShortForm}
                  </div>
                ) : (
                  'Select Token'
                )}

                <KeyboardArrowDownIcon />
              </div>
            </BorderGradientButton>
          </div>
          <div className='flex justify-between mt-2'>
            <div>${coinAmount > 0 ? coinAmount * payCoin?.marketPrice : 0}</div>
            <div>
              <button
                className='font-gilroy ml-1 sm:ml-2 text-orange-400 text-base font-normal'
                onClick={() => setCoinAmount(receiveCoinBalance)}
              >
                {receiveCoinBalance ? `${receiveCoinBalance} Max` : ''}
              </button>
            </div>
          </div>
        </div>

        {/* Swap Button */}
        <div
          className='absolute top-[37%] left-1/2 flex items-center justify-center p-2 cursor-pointer border border-[#FFFFFF66] rounded-xl bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-20'
          onClick={ClickChangeHandler}
        >
          <CachedIcon />
        </div>

        {/* Action Buttons */}
        <div className='w-[80%] sm:w-[480px] mx-auto text-center mt-4 flex gap-x-2'>
          {isAuthenticated ? (
            <div className='w-full'>
              {coinAmount !== 0 &&
              ((payCoinBalance <= 0 && receiveCoinBalance <= 0) ||
                coinAmount > payCoinBalance) ? (
                <GradientButton CustomCss='w-full md:w-full cursor-auto disabled opacity-75 font-extrabold text-3xl'>
                  {SwapModalData.MainButtonsText.InsufficientBalance}
                </GradientButton>
              ) : poolNotFound ? (
                <div>
                  <GradientButton
                    CustomCss='w-full md:w-full disabled opacity-75 font-extrabold text-3xl'
                    onClick={() => {
                      toast.error(
                        'No pool found for the selected pair. Please create one.'
                      )
                    }}
                  >
                    No Pool Found
                  </GradientButton>
                </div>
              ) : (
                <div className='w-full'>
                  {ClickedSwap ? (
                    <div
                      onClick={async () => {
                        setApprovalSuccess()
                        setSwapSuccess()
                        openModalWithSteps()
                        const res = await handleSwapApproval(
                          payCoin,
                          backendCanisterID
                        )
                        setApprovalSuccess(res)
                        if (res.success === true) {
                          const swapRes = await swapHandler()
                          setSwapSuccess(swapRes)
                        }
                      }}
                    >
                      <GradientButton CustomCss='w-full md:w-full font-extrabold text-3xl'>
                        {SwapModalData.MainButtonsText.ConfirmSwapping}
                      </GradientButton>
                    </div>
                  ) : (
                    <div className=''>
                      {coinAmount !== 0  ? (
                        <div
                          onClick={() => {
                            setClickSwap(true)
                          }}
                        >
                          <GradientButton CustomCss='w-full md:w-full font-extrabold text-3xl'>
                            {SwapModalData.MainButtonsText.SwapNow}
                          </GradientButton>
                        </div>
                      ) : (
                        <GradientButton CustomCss='w-full md:w-full cursor-auto disabled opacity-75 font-extrabold text-3xl'>
                          Enter an amount
                        </GradientButton>
                      )}
                    </div>
                  )}
                </div>
              )}
            </div>
          ) : (
            <GradientButton CustomCss='w-full md:w-full cursor-auto disabled opacity-75 font-extrabold text-3xl'>
              Connect wallet
            </GradientButton>
          )}

          <div className='relative flex items-center justify-center p-2 cursor-pointer border border-[#FFFFFF66] rounded-xl bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-20' onClick={handleSettings}>
            <SettingsIcon />
          </div>
        </div>
      </div>
      {searchToken2 && (
        <SearchToken
          setSearchToken={setSearchToken2}
          setPayToken={setPayCoin}
          setRecToken={setReceiveCoin}
          id={Id}
        />
      )}
       {settings && (
                        <>
                            <div className='fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm z-40' onClick={handleSettings}></div>
                            <div className='absolute w-[80%] sm:w-[480px] flex align-middle items-center justify-center h-full w-11/12'>
                                <SwapSetting />
                            </div>
                        </>
                    )}
    </section>
  )
}

export default Swap
