import React, { useEffect, useState } from 'react'
import { X } from 'lucide-react'
import { useSelector, useDispatch } from 'react-redux'
import BlueGradientButton from '../../buttons/BlueGradientButton'
import { toggleConfirm } from '../../reducer/PoolCreation'
import GradientButton from '../../buttons/GradientButton'
import { useAuth } from '../../components/utils/useAuthClient'
import { Principal } from '@dfinity/principal'
import CircularProgress from '@mui/material/CircularProgress'
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown'
import KeyboardArrowUpIcon from '@mui/icons-material/KeyboardArrowUp'
import CheckCircleOutlineIcon from '@mui/icons-material/CheckCircleOutline'
import CloseIcon from '@mui/icons-material/Close'
import { useNavigate } from 'react-router-dom'
const FinalizePool = ({ handleCreatePoolClick }) => {
  const { Tokens, Confirmation, TotalAmount, FeeShare } = useSelector(
    state => state.pool
  )
  const dispatch = useDispatch()
  const [confirmPool, setConfirmPool] = useState(false)
  const [poolCreated, setPoolCreated] = useState(false)
  const [final, setFinal] = useState(false)
  const [selectedTokenDetails, setSelectedTokenDetails] = useState()
  // const [poolData, setPoolData] = useState(null)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [approvalSuccess, setApprovalSuccess] = useState(false)
  const [subModel, setSubModel] = useState(1)
  const navigate = useNavigate()
  useEffect(() => {
    if (confirmPool && poolCreated) {
      setFinal(true)
    }
  }, [confirmPool, poolCreated])

  const InitialToken = Tokens[0]
  const RestTokens = Tokens.slice(1)

  const { backendActor, isAuthenticated } = useAuth()

  const createPoolHandler = () => {
    console.log('You clicked to create pool')

    // Check if Tokens array is valid
    if (!Tokens || Tokens.length === 0) {
      const errorMsg = 'No tokens available to create a pool'
      console.error(errorMsg)
      return Promise.reject(new Error(errorMsg))
    }

    // Process each token and map to pool_data
    const pool_data = Tokens.map((token, index) => {
      return new Promise(resolve => {
        console.log(`Processing token at index ${index}:`, token)
        if (token.weights <= 0) {
          throw new Error(
            `Invalid weight for token ${token.ShortForm}: ${token.weights}`
          )
        }
        if (token.Amount <= 0) {
          throw new Error(
            `Invalid balance for token ${token.ShortForm}: ${token.Amount}`
          )
        }
        if (token.currencyAmount <= 0) {
          throw new Error(
            `Invalid value for token ${token.ShortForm}: ${token.currencyAmount}`
          )
        }
        // Check if token properties exist
        if (
          token.weights === undefined ||
          token.Amount === undefined ||
          token.currencyAmount === undefined ||
          token.CanisterId === undefined
        ) {
          const errorMsg = `Missing data for token ${token.ShortForm}`
          console.error(errorMsg)
          return resolve(null) // Resolve with null to indicate invalid token
        }

        // Validate and parse weight
        const weightStr = token.weights.toString().replace(/[^0-9.-]+/g, '')
        const weight = parseFloat(weightStr)
        console.log(`Token ${token.ShortForm} weightStr:`, weightStr)
        console.log(`Token ${token.ShortForm} parsed weight:`, weight)

        if (!Number.isFinite(weight)) {
          const errorMsg = `Invalid weight value for token ${token.ShortForm}: ${token.weights}`
          console.error(errorMsg)
          return resolve(null)
        }

        // Convert weight to an integer (e.g., scaled by 100)
        const normalizedWeight = BigInt(Math.round(weight))
        console.log(
          `Token ${token.ShortForm} normalized weight (scaled):`,
          normalizedWeight
        )

        // Validate and parse amount
        const decimals = token.metaData?.decimals || 1 // Default to 1 if decimals is undefined
        console.log("decimals", decimals)
        const amountStr = token.Amount.toString().replace(/[^0-9.-]+/g, '')
         const amount = parseFloat(amountStr * Math.pow(10, decimals));
        console.log(`Token ${token.ShortForm} amountStr:`, amountStr)
        console.log(`Token ${token.ShortForm} parsed amount:`, amount)

        if (!Number.isFinite(amount)) {
          const errorMsg = `Invalid amount value for token ${token.ShortForm}: ${token.Amount}`
          console.error(errorMsg)
          return resolve(null)
        }
        const balance = BigInt(Math.round(amount));
        console.log(`Token ${token.ShortForm} balance (BigInt):`, balance)

        // Validate and parse currency amount
        const currencyAmountStr = token.currencyAmount
          .toString()
          .replace(/[^0-9.-]+/g, '')
        const currencyAmount = parseFloat(currencyAmountStr)
        console.log(
          `Token ${token.ShortForm} currencyAmountStr:`,
          currencyAmountStr
        )
        console.log(
          `Token ${token.ShortForm} parsed currencyAmount:`,
          currencyAmount
        )

        if (!Number.isFinite(currencyAmount)) {
          const errorMsg = `Invalid currency amount for token ${token.ShortForm}: ${token.currencyAmount}`
          console.error(errorMsg)
          return resolve(null)
        }
        const value = BigInt(Math.round(currencyAmount))
        console.log(`Token ${token.ShortForm} value (BigInt):`, value)

        // Convert CanisterId to Principal
        let CanisterId
        try {
          CanisterId = Principal.fromText(token.CanisterId)
          console.log(
            `Token ${token.ShortForm} CanisterId (Principal):`,
            CanisterId.toText()
          )
        } catch (error) {
          const errorMsg = `Invalid Canister ID for token ${token.ShortForm}: ${token.CanisterId}`
          console.error(errorMsg)
          return resolve(null)
        }

        // Construct the pool data object
        const poolData = {
          weight: normalizedWeight, // Ensure weight is an integer
          balance: balance,
          value: value,
          image: token.ImagePath || '',
          token_name: token.ShortForm || 'Unnamed Token',
          ledger_canister_id: CanisterId
        }

        resolve(poolData)
      })
    })

    // Wait for all token processing to complete
    return Promise.allSettled(pool_data)
      .then(async results => {
        const validPoolData = results
          .filter(data => data.status === 'fulfilled' && data.value !== null)
          .map(data => data.value)

        console.log('Valid pool data:', validPoolData)

        if (validPoolData.length === 0) {
          const errorMsg = 'No valid tokens available for creating a pool.'
          console.error(errorMsg)
          return Promise.reject(new Error(errorMsg))
        }

        if (FeeShare === undefined || FeeShare === null) {
          const errorMsg = 'FeeShare is undefined or null'
          console.error(errorMsg)
          return Promise.reject(new Error(errorMsg))
        }

        const feeShareStr = FeeShare.toString().replace(/[^0-9.-]+/g, '')
        const swap_fee = parseFloat(feeShareStr)
        console.log('Parsed swap_fee:', swap_fee)

        if (!Number.isFinite(swap_fee)) {
          const errorMsg = `Invalid swap fee: ${FeeShare}`
          console.error(errorMsg)
          return Promise.reject(new Error(errorMsg))
        }

        const poolDetails = {
          pool_data: validPoolData,
           swap_fee: BigInt(Math.round(swap_fee * 10))
        } // Ensure swap_fee is an integer
        console.log('Final poolDetails to be sent to backend:', poolDetails)

        if (!backendActor || !backendActor.create_pools) {
          const errorMsg =
            'Backend actor is not available or create_pools method is missing'
          console.error(errorMsg)
          return Promise.reject(new Error(errorMsg))
        }

        return backendActor.create_pools(poolDetails).then(result => {
          console.log('Backend response:', result)

          if (result && 'Ok' in result) {
            console.log('Pool created successfully')
            setPoolCreated(true)
            return result
          } else if (result && result.Err) {
            console.error('Error creating pool:', result.Err)
            return Promise.reject(new Error(result.Err))
          } else {
            const errorMsg = `Unexpected response from backend: ${JSON.stringify(
              result
            )}`
            console.error(errorMsg)
            return Promise.reject(new Error(errorMsg))
          }
        })
      })
      .catch(error => {
        console.error('Error while creating pool:', error)
        return Promise.reject(error)
      })
  }

  const finalPoolCreationghanlder = () => {
    setIsModalOpen(true)
    const backendCanisterID = process.env.CANISTER_ID_VALUESWAP_BACKEND
    console.log('Backend Canister ID:', backendCanisterID)

    console.log('Starting handleCreatePoolClick')

    handleCreatePoolClick(backendCanisterID)
      .then(poolClickResult => {
        console.log('handleCreatePoolClick(approval) result:', poolClickResult)

        if (poolClickResult.success) {
          console.log('Starting createPoolHandler')
          setApprovalSuccess(true)
          // Call createPoolHandler and chain the Promise
          return createPoolHandler().then(createPoolResult => {
            console.log('createPoolHandler result:', createPoolResult)

            if (createPoolResult || createPoolResult.Ok) {
              setConfirmPool(true)
              console.log('Pool creation confirmed')
            } else {
              console.error('createPoolHandler failed:', createPoolResult.error)
              // Optionally, you can reject the Promise to handle it in the catch block
              return Promise.reject(new Error(createPoolResult.error))
            }
          })
        } else {
          console.error(
            'handleCreatePoolClick indicated failure:',
            poolClickResult.error
          )
          // Optionally, handle partial failures
          if (poolClickResult.details) {
            poolClickResult.details.forEach(detail => {
              console.error(
                `Token ${detail.token.CanisterId} failed: ${detail.error}`
              )
            })
          }
          // Reject the Promise to skip createPoolHandler
          return Promise.reject(new Error(poolClickResult.error))
        }
      })
      .catch(error => {
        console.error('handleCreatePoolClick failed:', error)
        // Optionally, you can handle the error here (e.g., show a notification to the user)
        // Example using a toast notification:
        // toast.error(`Pool creation failed: ${error.message}`);
      })
  }

  return (
    <div className='flex z-50 justify-center fixed inset-0 bg-opacity-50 backdrop-blur-sm py-10 overflow-y-scroll'>
      <div className='h-fit xl:w-5/12 lg:w-6/12 md:w-7/12 sm:w-8/12 w-11/12 border rounded-xl flex flex-col gap-2 bg-[#05071D] my-auto'>
        <div className='md:w-[64%] w-[62%] flex place-self-end items-center justify-between mx-4'>
          <span className='font-gilroy font-medium md:text-2xl text-xl py-4'>
            Analyse Pair
          </span>
          <div
            className='cursor-pointer'
            onClick={() => {
              console.log('dispatched called')
              dispatch(
                toggleConfirm({
                  value: false,
                  page: 'Final Page'
                })
              )
              console.log('dispatched finished')
            }}
          >
            <X />
          </div>
        </div>
        <div className='border border-transparent font-bold custom-height-3 bg-gradient-to-r from-transparent via-[#00308E] to-transparent w-full mx-auto mb-4'></div>

        {Tokens.map((token, index) => (
          <div className='mx-3 sm:mx-10' key={index}>
            <div className='flex justify-between items-center font-gilroy'>
              <div className='flex justify-evenly items-center gap-2'>
                <BlueGradientButton>
                  <img src={token.ImagePath} alt='' className='h-12 w-12' />
                </BlueGradientButton>

                <div>{token.ShortForm}</div>

                <span className='bg-[#3E434B] p-1 rounded-lg px-3'>
                  {token.weights} %
                </span>
              </div>

              <div className='flex flex-col justify-end items-end'>
                <div className='text-center'>
                  <span className='font-normal leading-5 text-xl sm:text-3xl  py-1 inline-block'>
                    {Number.isFinite(parseFloat(token.Amount))
                      ? token.Amount
                      : 'N/A'}
                  </span>
                </div>
                <span className='text-sm sm:text-base font-normal '>
                  $
                  {Number.isFinite(parseFloat(token.currencyAmount))
                    ? token.currencyAmount
                    : 'N/A'}
                </span>
              </div>
            </div>
          </div>
        ))}
        <div className='border border-transparent font-bold custom-height-3 bg-gradient-to-r from-transparent via-[#00308E] to-transparent w-full mx-auto mb-4'></div>

        <div className='flex justify-between font-gilroy font-light text-base mx-10'>
          <span>Total</span>

          <span>${TotalAmount}</span>
        </div>

        <h1 className='text-center font-gilroy font-medium text-xl leading-5 '>
          Overview
        </h1>
        <div className='border border-transparent font-bold custom-height-3 bg-gradient-to-r from-transparent via-[#00308E] to-transparent w-full mx-auto mb-4'></div>

        <div className='flex justify-between font-gilroy font-normal text-sm sm:text-base mx-5 sm:mx-10'>
          <span>Pool Symbol</span>

          <div className='leading-6 inline-block items-center text-center'>
            <span className='inline'>{InitialToken.ShortForm}</span>
            {RestTokens.map((token, index) => (
              <span key={index}> / {token.ShortForm}</span>
            ))}

            <span className='inline mx-1 '>: :</span>

            <span className='inline'>{InitialToken.weights}</span>

            {RestTokens.map((token, index) => (
              <span key={index}> / {token.weights}</span>
            ))}
          </div>
        </div>
        <div className='flex justify-between font-gilroy font-normal text-sm sm:text-base mx-5 sm:mx-10'>
          <span>Pool Name</span>

          <div className='leading-6 inline-block items-center text-center'>
            <span className='inline'>{InitialToken.metaData.symbol}</span>
            {RestTokens.map((token, index) => (
              <span key={index}> / {token.metaData.symbol}</span>
            ))}

            <span className='inline mx-1 '>: :</span>

            <span className='inline'>{InitialToken.weights}</span>

            {RestTokens.map((token, index) => (
              <span key={index}> / {token.weights}</span>
            ))}
          </div>
        </div>

        <div className='flex justify-between font-gilroy font-normal text-sm sm:text-base mx-5 sm:mx-10'>
          <span>Pool Fee Share</span>

          <div className='leading-6 inline-block items-center text-center'>
            {FeeShare} %
          </div>
        </div>

        <div className={`mx-10 mb-4`}>
          <div
            className={`${confirmPool ? 'hidden' : 'block'}`}
            onClick={finalPoolCreationghanlder}
          >
            <GradientButton CustomCss={` w-full md:w-full`}>
              Confirm and Create Pool
            </GradientButton>
          </div>

          <div
            className={`${confirmPool ? 'block enabled' : 'hidden disabled'} ${
              poolCreated ? 'hidden disabled' : 'block '
            }`}
            onClick={() => {
              setPoolCreated(true)
              console.log('Tokens in the pool Data:->', Tokens)
            }}
          >
            <GradientButton CustomCss={` w-full md:w-full`}>
              Supply Funds
            </GradientButton>
          </div>

          <div className={`${final ? 'block' : 'hidden'}`}>
            <GradientButton CustomCss={` w-full md:w-full`}>
              View Pool
            </GradientButton>
          </div>
        </div>
      </div>

      {/*steps confirmation model */}
      {isModalOpen ? (
        <div className='fixed inset-0  bg-black bg-opacity-50 flex justify-center items-center z-50'>
          <div className='p-6 pb-16  xl:w-4/12 lg:w-5/12 md:w-6/12 sm:w-7/12 w-11/12 border-2 border-[#86828280] bg-[#182030] mt-10 rounded-lg shadow-lg text-white  mx-auto relative'>
            <button
              className='absolute top-5 right-10 text-gray-400 hover:text-gray-300'
              onClick={() => setIsModalOpen(false)}
            >
              <CloseIcon />
            </button>

            <h2 className='text-xl font-medium mb-4 text-center font-gilroy'>
              Pool Creation Details
            </h2>
            {/* <p className="text-gray-400 mb-6">
            You can swap directly without depositing, because you have sufficient balance in the Swap pool.
          </p> */}

            <div className='flex flex-col gap-y-6 font-gilroy'>
              <div className='flex gap-x-4 '>
                <div className='flex justify-center items-center '>
                  {approvalSuccess ? (
                    <CheckCircleOutlineIcon style={{ color: 'green' }} />
                  ) : (
                    <CircularProgress size='20px' />
                  )}
                </div>
                <div className='flex flex-col border rounded-lg  py-2 border-gray-600 bg-[#30303080]  w-full'>
                  <div className='flex justify-between  w-full px-4'>
                    <div className='flex gap-x-4 font-gilroy pb-1'>
                      <span>1. Approve Tokens</span>
                      {/* <span></span> */}
                    </div>
                    <div onClick={() => setSubModel(1)}>
                      {subModel == 1 ? (
                        <KeyboardArrowUpIcon />
                      ) : (
                        <KeyboardArrowDownIcon />
                      )}
                    </div>
                  </div>
                  {Tokens.map(token => (
                    <div
                      className={` ${
                        subModel == 1 ? 'flex flex-col' : 'hidden'
                      }`}
                    >
                      <hr className=' border-[#FFFFFF4D] w-full' />
                      <div className='flex  justify-between w-full font-extralight text-sm px-4 pt-1 text-[#FFFFFFBF]'>
                        <span>Amount</span>
                        <span className='flex gap-2 items-center'>
                          <img
                            src={token.ImagePath}
                            alt=''
                            className='w-4 h-4'
                          />
                          {token.Amount}
                        </span>
                      </div>
                      <div className='flex justify-between w-full font-extralight text-sm px-4 pb-1 text-[#FFFFFFBF]'>
                        <span>Canister Id</span>
                        <span>{token.CanisterId}</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              <div className='flex gap-x-4 '>
                <div className='flex justify-center items-center'>
                  {approvalSuccess ? (
                    <CheckCircleOutlineIcon style={{ color: 'green' }} />
                  ) : (
                    <CircularProgress size='20px' />
                  )}
                </div>
                <div className='flex flex-col border rounded-lg py-2 border-gray-600 bg-[#30303080]  w-full'>
                  <div className='flex justify-between  w-full'>
                    <div className='flex gap-x-4 pb-1 px-4'>
                      <span>2. Deposit Tokens</span>
                    </div>
                    <div onClick={() => setSubModel(2)}>
                      {subModel == 2 ? (
                        <KeyboardArrowUpIcon />
                      ) : (
                        <KeyboardArrowDownIcon />
                      )}
                    </div>
                  </div>
                  {Tokens.map(token => (
                    <div
                      className={` ${
                        subModel == 2 ? 'flex flex-col' : 'hidden'
                      }`}
                    >
                      <hr className=' border-gray-500' />
                      <div className='flex justify-between w-full font-extralight text-sm px-4 pt-1 text-[#FFFFFFBF]'>
                        <span>Amount</span>
                        <span className='flex gap-2 items-center'>
                          <img
                            src={token.ImagePath}
                            alt=''
                            className='w-4 h-4'
                          />
                          {token.Amount}
                        </span>
                      </div>
                      <div className='flex justify-between w-full font-extralight text-sm px-4 pb-1 text-[#FFFFFFBF]'>
                        <span>Canister Id</span>
                        <span>{token.CanisterId}</span>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
              <div className='flex gap-x-4 '>
                <div className='flex justify-center items-center'>
                  {confirmPool ? (
                    <CheckCircleOutlineIcon style={{ color: 'green' }} />
                  ) : (
                    <CircularProgress size='20px' />
                  )}
                </div>
                <div className='flex flex-col border rounded-lg px-4 py-2 border-gray-600 bg-[#30303080] w-full'>
                  <div className='flex justify-between  w-full'>
                    <div className='flex gap-x-4'>
                      <span>3. Pool Creation Completed</span>
                    </div>
                  </div>
                </div>
              </div>

              <div className='flex gap-x-4  justify-center'>
                {confirmPool && (
                  <div
                    className={final ? 'block' : 'hidden'}
                    onClick={() => navigate('/valueswap/portfolio')}
                  >
                    <GradientButton CustomCss='w-full md:w-full'>
                      View Pool
                    </GradientButton>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      ) : (
        ''
      )}
    </div>
  )
}

export default FinalizePool
