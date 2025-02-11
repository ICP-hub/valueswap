import React, { useEffect, useMemo, useState } from 'react';
import { Bolt, ChevronDown, ChevronUp, Dot, Info } from 'lucide-react';
import BlueGradientButton from '../buttons/BlueGradientButton';
import GradientButton from '../buttons/GradientButton';
import ImpactButton from '../buttons/ImpactButton';
import SearchToken from './SearchToken';
import { SwapModalData } from '../TextData';
import { useNavigate } from 'react-router-dom';
import SwapSetting from './SwapSetting';
import { useAuth } from '../components/utils/useAuthClient';
import { Principal } from '@dfinity/principal';
import ErrorOutlineOutlinedIcon from '@mui/icons-material/ErrorOutlineOutlined';
import CircularProgress from '@mui/material/CircularProgress';
import KeyboardArrowDownIcon from '@mui/icons-material/KeyboardArrowDown';
import KeyboardArrowUpIcon from '@mui/icons-material/KeyboardArrowUp';
import CheckCircleOutlineIcon from '@mui/icons-material/CheckCircleOutline';
import { toast } from 'react-toastify';
import CloseIcon from '@mui/icons-material/Close';
import BorderGradientButton from '../buttons/BorderGradientButton';
import DialogBox from './Dialouge';

const Swap = () => {
    const navigate = useNavigate();
    const [PayCoin, setPayCoin] = useState(null);
    const [RecieveCoin, setRecieveCoin] = useState(null);
    const [Message, setMessage] = useState('');
    const [show1, setShow1] = useState(false);
    const [show2, setShow2] = useState(false);
    const [show3, setShow3] = useState(false);
    const [changePayCoin, setChangePayCoin] = useState("M5 13.5L9 9.5M5 13.5L1 9.5M5 13.5V1");
    const [CoinAmount, setCoinAmount] = useState();
    const [AmountToPay, setAmountToPay] = useState(0.0);
    const [settings, setSettings] = useState(false);
    const [changeRecieveCoin, setChangeRecieveCoin] = useState("M13 1L9 5M13 1L17 5M13 1L13 13.5");
    const [bothCoins, setBothCoins] = useState(false);
    const [searchToken1, setSearchToken1] = useState(false);
    const [searchToken2, setSearchToken2] = useState(false);
    const [id, setId] = useState(0);
    const [ClickedSwap, setClickSwap] = useState(false);
    const [payCoinBalance, setPayCoinBalance] = useState(null); // New state for PayCoin balance
    const [recieveCoinBalance, setRecieveCoinBalance] = useState(null); // New state for RecieveCoin balance
    const [recieveValue, setReciveValue] = useState(0)
    const { backendActor, principal, getBalance, isAuthenticated, createTokenActor } = useAuth();
    const [isModalOpen, setIsModalOpen] = useState(false); // Modal visibility state
    const [approvalSuccess, setApprovalSuccess] = useState(false);
    const [swapSuccess, setSwapSuccess] = useState(false);
    const [subModel, setSubModel] = useState(1)
    
    // const { Tokens, Confirmation, TotalAmount, FeeShare } = useSelector((state) => state.pool);


    useEffect(() => {
        if (PayCoin && RecieveCoin) {
            setBothCoins(true);

        } else {
            setBothCoins(false);
        }
    }, [PayCoin, RecieveCoin]);


    useEffect(() => {
        const getSwapValue = async () => {
           
            if (CoinAmount) {
                const amount = parseFloat(CoinAmount)
                const swapValue = await backendActor.pre_compute_swap({
                    token1_name: PayCoin.ShortForm,
                    token_amount: amount,
                    token2_name: RecieveCoin.ShortForm,
                    ledger_canister_id1: Principal.fromText(RecieveCoin.CanisterId),
                    ledger_canister_id2: Principal.fromText(RecieveCoin.CanisterId)
                })
                console.log("swapValue", swapValue)
                setReciveValue(swapValue[1])
                
            } else {
                console.log("no coin Amount enter")
            }
            if (PayCoin) {
                getBalance(PayCoin.CanisterId)
                    .then(balance => {
                        setPayCoinBalance(Number(balance) / 100000000);
                    })
                    .catch((err) => console.log(err));
                // console.log("Balance", payCoinBalance);
            }
        }
        getSwapValue()
    }, [PayCoin, getBalance, CoinAmount, RecieveCoin]);


    useEffect(() => {
        if (RecieveCoin) {
            getBalance(RecieveCoin?.CanisterId).then(balance => {
                setRecieveCoinBalance(Number(balance) / 100000000);
            }).catch((err) => console.log(err));;
        }
    }, [RecieveCoin, getBalance]);

    function ClickedChange() {
        let Temp = changePayCoin;
        setChangePayCoin(changeRecieveCoin);
        setChangeRecieveCoin(Temp);
        Temp = PayCoin;
        setPayCoin(RecieveCoin);
        setRecieveCoin(Temp);
    }

    const handleChangeAmount = (e) => {
        let number = e.target.value;
        if (!isNaN(number) && Number(number) >= 0) {
            setCoinAmount(number);
        } else {
            setCoinAmount(0);
        }
    };

    const handleSettings = () => {
        setSettings((prev) => !prev);
    };




    //approval foe swaping

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



            console.log("init metaData", metaData);
            console.log("init decimals", decimals);
            console.log("init fee", fee);
            console.log("init amount", amount);
            console.log("init balance", balance);

            if (balance >= amount + fee) {
                const transaction = {
                    amount: BigInt(amount + fee),  // Approving amount (including fee)
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
    const backendCanisterID = process.env.CANISTER_ID_VALUESWAP_BACKEND;
    const handleSwapApproval = async (PayCoin, backendCanisterID) => {
        return new Promise((resolve, reject) => {
            if (!PayCoin) {
                return resolve({ success: false, error: "No token to process" })
            }

            if (!PayCoin.CanisterId || !CoinAmount) {
                const errorMsg = `Invalid token data: ${JSON.stringify(PayCoin)}`;
                console.log(errorMsg);
                return resolve({ success: false, error: errorMsg, token: PayCoin })
            }

            createTokenActor(PayCoin.CanisterId).then((tokenActor) => {
                // console.log("tokenActor", tokenActor);
                return transferApprove(
                    CoinAmount,
                    PayCoin.CanisterId,
                    backendCanisterID,
                    tokenActor
                )
            })
                .then((approvalResult) => {
                    if (!approvalResult.success) {
                        console.error(`Approval failed for token: ${PayCoin.CanisterId}`, approvalResult.error);
                        // toast.error("approve failed")
                        return resolve({ success: false, error: approvalResult.error, token: PayCoin });
                    } else {
                        console.log(`Approval successful for token: ${PayCoin.CanisterId}`);
                        setApprovalSuccess(true)
                        return resolve({ success: true, data: approvalResult.data, token: PayCoin })
                    }
                }).catch((error) => {
                    console.error("Error in handleSwapApproval:", error);
                    return resolve({ success: false, error: error.message })
                })
        })
    };




    const swapHandler = async () => {
        console.log("Click on swap", { CoinAmount, PayCoin, RecieveCoin });

        if (CoinAmount === undefined || CoinAmount === null) {
            console.error("CoinAmount is undefined or null");
            return error;
        }
        let amount;
        try {
            amount = parseFloat(CoinAmount);
        } catch (error) {
            console.error("Invalid CoinAmount:", CoinAmount, error);
            return error;
        }
        if (!PayCoin || !PayCoin.ShortForm) {
            console.error("PayCoin is invalid:", PayCoin);
            return error;
        }
        if (!RecieveCoin || !RecieveCoin.ShortForm) {
            console.error("RecieveCoin is invalid:", RecieveCoin);
            return error;
        }
        if (!backendActor || !backendActor.compute_swap) {
            console.error("backendActor is not available or compute_swap method is missing");
            return error;
        }

        try {
            console.log("Calling backendActor.compute_swap with:", {
                token_amount: amount,
                token1_name: PayCoin.ShortForm,
                token2_name: RecieveCoin.ShortForm,
                ledger_canister_id1: PayCoin.CanisterId,
                ledger_canister_id2: RecieveCoin.CanisterId
            });
            const res = await backendActor.compute_swap({
                token1_name: PayCoin.ShortForm,
                token_amount: amount,
                token2_name: RecieveCoin.ShortForm,
                ledger_canister_id1: Principal.fromText(PayCoin.CanisterId),
                ledger_canister_id2: Principal.fromText(RecieveCoin.CanisterId)
            });

            console.log("Response from compute_swap:", res);
            if(res.Ok){
                getSwapValue()
            }
            console.log("slipage:", recieveValue);

            if (res.Ok == null) {
                console.log("Swap successful");
                // setSwapSuccess(true)
                toast.success("swap complete")
                // navigate('/valueswap/transaction-successfull');
                return res;
            } else if (res && res.Err) {
                console.error("Swap failed with error:", res.Err, res);
                return res;
            } else {
                console.error("Unexpected response from compute_swap:", res);
                return res;
            }
        } catch (error) {
            console.error("Error while calling swap function:", error);
            return res;

        }
    };

    const openModalWithSteps = async () => {
        // Define steps for the modal
        // const steps = [
        //     { title: 'Approve Swap', completed: approvalSuccess },
        //     { title: 'Deposit', completed: approvalSuccess },
        //     { title: 'Compute Swap', completed: swapSuccess },
        // ];

        // Set modal steps and open the modal
        // setModalSteps(steps);
        setIsModalOpen(true);
    };
    return (
        <div className='px-4 md:px-0'>
            <div className='flex justify-center my-auto flex-col'>
                <div className='relative align-middle max-w-[1200px] flex flex-col justify-center mt-12 p-6 bg-gradient-to-b from-[#3E434B] to-[#02060D] border sm:mx-auto rounded-lg '>
                    <div className='w-[64%] sm:w-[58%] place-self-end flex justify-between '>
                        <span className='font-gilroy font-light text-3xl'>{SwapModalData.Heading}</span>
                        <Bolt size={30} className='cursor-pointer' onClick={handleSettings} />
                    </div>
                    {settings && (
                        <>
                            <div className='fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm z-40' onClick={handleSettings}></div>
                            <div className='absolute flex align-middle items-center justify-center h-full w-11/12'>
                                <SwapSetting />
                            </div>
                        </>
                    )}
                    <div className='mx-auto sm:mx-4 w-full flex justify-between items-center'>
                        {PayCoin ? (
                            <div className='flex flex-col font-gilroy font-normal gap-2'>
                                <span className='text-base font-medium'>{SwapModalData.PaySection.Heading}</span>
                                <span className='text-3xl md:text-4xl'>
                                    <input
                                        type="number"
                                        className='bg-transparent w-64 outline-none hide-arrows w-full'
                                        placeholder='0.0'
                                        value={CoinAmount}
                                        onChange={handleChangeAmount}
                                    />
                                </span>
                                <div>
                                    <span className='text-base font-normal'>
                                        {SwapModalData.PaySection.Balance}: {payCoinBalance !== null ? parseFloat(payCoinBalance) : 'Loading...'}
                                        <button className='font-gilroy ml-1 sm:ml-2 text-orange-400' onClick={() => {
                                            setCoinAmount(parseFloat(payCoinBalance));
                                        }}>
                                            {SwapModalData.PaySection.Max}
                                        </button>
                                    </span>
                                </div>
                            </div>
                        ) : (
                            <div className='flex flex-col font-gilroy font-normal gap-2'>
                                <span className='text-base font-medium'>{SwapModalData.PaySection.Heading}</span>
                                <span className='text-3xl md:text-4xl'>0</span>
                                <span className='text-sm sm:text-base font-medium'>{SwapModalData.PaySection.Balance}: {SwapModalData.PaySection.NoTokenSelectBalanceMessage}</span>
                            </div>
                        )}

                        <div>
                            {!PayCoin ? (
                                <div>
                                    <div className='flex sm:mr-12 items-center gap-2 ' onClick={() => {
                                        setId(1);
                                        setSearchToken1(!searchToken1);
                                    }}>
                                        <BlueGradientButton customCss={'px-2 md:w-44 sm:px-4 py-1 sm:py-3 font-gilroy md:font-light'}>
                                            <div className='flex text-sm sm:text-base items-center gap-1' >
                                                {SwapModalData.PaySection.TokenSelectButtonText}
                                                <span className='cursor-pointer' >
                                                    <ChevronDown />
                                                </span>
                                            </div>
                                        </BlueGradientButton>
                                    </div>
                                    <div>

                                        {searchToken1 && <SearchToken setSearchToken={setSearchToken1} setPayToken={setPayCoin} setRecToken={setRecieveCoin} id={id} />}
                                    </div>
                                </div>
                            ) : (
                                <div className='flex flex-col gap-1'>
                                    <div className='flex sm:mr-12 items-center place-self-end gap-2'>
                                        <BlueGradientButton customCss={'disabled px-2 py-2 normal-cursor'}>
                                            <img src={PayCoin.ImagePath} alt="" className='h-6 w-6 transform scale-150' />
                                        </BlueGradientButton>

                                        <div className='font-gilroy font-normal text-2xl'>
                                            {PayCoin.ShortForm}
                                        </div>
                                        {!searchToken1 ? (
                                            <span className='cursor-pointer' onClick={() => {
                                                setId(1);
                                                setSearchToken1(!searchToken1);
                                            }}>
                                                <ChevronDown />
                                            </span>
                                        ) : (
                                            <span className='cursor-pointer' onClick={() => {
                                                setSearchToken1(!searchToken1);
                                            }}>
                                                <ChevronUp />
                                            </span>
                                        )}
                                        {searchToken1 && <SearchToken setSearchToken={setSearchToken1} setPayToken={setPayCoin} setRecToken={setRecieveCoin} id={id} />}
                                    </div>
                                    <span className='font-gilroy font-normal text-center'>
                                        ${CoinAmount ? (PayCoin.marketPrice * CoinAmount).toLocaleString() : 0}
                                    </span>
                                </div>
                            )}
                        </div>
                    </div>

                    <div className='flex flex-col justify-center items-center mt-8'>
                        <div className='border-b border-white w-11/12'></div>
                        <div className='bg-[#000711] rounded-xl p-4 xl:w-1/12 lg:w-1/6 -mt-6 cursor-pointer flex flex-col items-center' onClick={ClickedChange}>
                            <svg width="18" height="15" viewBox="0 0 18 15" fill="none" xmlns="http://www.w3.org/2000/svg">
                                <path d="M5 13.5L9 9.5L5 13.5ZM5 13.5L1 9.5L5 13.5ZM5 13.5V1V13.5Z" fill="#000711" />
                                <path d={`${changeRecieveCoin}`} stroke="#0057FF" />
                                <path d="M13 1L9 5L13 1ZM13 1L17 5L13 1ZM13 1L13 13.5L13 1Z" fill="#000711" />
                                <path d={`${changePayCoin}`} stroke="white" strokeOpacity="0.3" />
                            </svg>
                        </div>
                    </div>

                    <div className='mx-auto sm:mx-4 w-full flex justify-between items-center'>
                        {RecieveCoin ? (
                            <div className='flex flex-col font-gilroy font-normal gap-2'>
                                <span className='text-base font-medium'>{SwapModalData.RecieveSection.Heading}</span>
                                <span className='text-3xl md:text-4xl'>{CoinAmount ? recieveValue.toFixed(8) : 0}</span>
                                <span className='text-sm sm:text-base font-normal'>
                                    {SwapModalData.RecieveSection.Balance}: {recieveCoinBalance !== null ? parseFloat(recieveCoinBalance) : 'Loading...'}
                                </span>
                            </div>
                        ) : (
                            <div className='flex flex-col font-gilroy font-normal gap-2'>
                                <span className='text-base font-medium'>{SwapModalData.RecieveSection.Heading}</span>
                                <span className='text-3xl md:text-4xl'>0</span>
                                <span className='text-sm sm:text-base font-normal'> {SwapModalData.RecieveSection.Balance}:  {SwapModalData.RecieveSection.NoTokenSelectBalanceMessage}</span>
                            </div>
                        )}

                        <div>
                            {!RecieveCoin ? (
                                <div>
                                    <div className='flex sm:mr-12 items-center place-self-end gap-2' onClick={() => {
                                        setId(2);
                                        setSearchToken2(!searchToken2);
                                    }}>
                                        <BlueGradientButton customCss={'px-2 md:w-44 sm:px-4 py-1 sm:py-3 font-gilroy md:font-light'}>
                                            <div className='flex text-sm sm:text-base items-center gap-1'
                                            >
                                                {SwapModalData.RecieveSection.TokenSelectButtonText}
                                                <span className='cursor-pointer'>
                                                    <ChevronDown />
                                                </span>
                                            </div>
                                        </BlueGradientButton>
                                    </div>
                                    <div>
                                        {searchToken2 && <SearchToken setSearchToken={setSearchToken2} setRecToken={setRecieveCoin} setPayToken={setPayCoin} id={id} />}
                                    </div>
                                </div>
                            ) : (
                                <div className='flex flex-col gap-1'>
                                    <div className='flex sm:mr-12 items-center place-self-end gap-2'>
                                        <BlueGradientButton customCss={'disabled px-2 py-2 normal-cursor'}>
                                            <img src={RecieveCoin?.ImagePath} alt="" className='h-6 w-6 transform scale-150' />
                                        </BlueGradientButton>

                                        <div className='font-gilroy font-normal text-2xl'>
                                            {RecieveCoin?.ShortForm}
                                        </div>
                                        {!searchToken2 ? (
                                            <span className='cursor-pointer' onClick={() => {
                                                setSearchToken2(!searchToken2);
                                                setId(2);
                                            }}>
                                                <ChevronDown />
                                            </span>
                                        ) : (
                                            <span className='cursor-pointer' onClick={() => {
                                                setSearchToken2(!searchToken2);
                                                setId(2);
                                            }}>
                                                <ChevronUp />
                                            </span>
                                        )}
                                        {searchToken2 && <SearchToken setSearchToken={setSearchToken2} setRecToken={setRecieveCoin} setPayToken={setPayCoin} id={id} />}
                                    </div>
                                    <span className='font-gilroy font-normal text-center'>
                                        ${CoinAmount ? (((PayCoin?.marketPrice * CoinAmount) / RecieveCoin?.marketPrice) * RecieveCoin?.marketPrice).toLocaleString() : 0}
                                    </span>
                                </div>
                            )}
                        </div>
                    </div>

                    {bothCoins && (
                        <div className='w-full mx-auto'>
                            <div className='flex justify-between items-center my-3'>
                                {/* <div className='flex items-center'>
                                    <Dot color='#F7931A' />
                                    <span>{SwapModalData.bothCoinsPresent.Price}</span>
                                </div>

                                <div className='font-gilroy font-medium text-sm sm:text-base'>
                                    {`1 CT = 0.0025 ETH (12.58$)`}
                                </div> */}
                            </div>
{/* 
                            <div className='flex justify-between items-center my-3'>
                                <div className='flex items-center'>
                                    <Dot color='#F7931A' />
                                    <span>{SwapModalData.bothCoinsPresent.GasFees}</span>
                                </div>

                                <div className='font-gilroy font-medium text-sm sm:text-base'>
                                    {`0.000052 ETH  ($0.1656)`}
                                </div>
                            </div> */}

                            {/* <div className='w-full'>
                                {ClickedSwap && (
                                    <div className='flex flex-col gap-8 my-4 rounded-lg'>
                                        <div className='flex justify-between'>
                                            <div className='flex items-center'>
                                                <Dot color='#F7931A' />
                                                <span>{SwapModalData.ClickedSwapData.MinimumRecieved}</span>
                                            </div>

                                            <div className='font-gilroy font-medium text-base'>10.5580 CT</div>
                                        </div>
                                        <div className='flex justify-between items-center'>
                                            <div className='flex items-center'>
                                                <Dot color='#F7931A' />
                                                <span className='relative'>{SwapModalData.ClickedSwapData.OverallSlippage}</span>
                                            </div>

                                            <div><ImpactButton customCss={`font-bold`} Impact={'Positive'}>10%</ImpactButton></div>
                                        </div>
                                        <div className='flex justify-between items-center'>
                                            <div className='flex items-center'>
                                                <Dot color='#F7931A' />
                                                <span>{SwapModalData.ClickedSwapData.LiquidityProviderIncentive}</span>
                                            </div>
                                            <div className='font-gilroy font-medium text-base'>
                                                0.000056 ETH
                                            </div>
                                        </div>
                                    </div>
                                )}
                            </div> */}

                            {isAuthenticated ? <div className='w-full'>
                                {(payCoinBalance <= 0 && recieveCoinBalance <= 0) || CoinAmount > payCoinBalance ? (
                                    <GradientButton CustomCss={'w-full md:w-full cursor-auto disabled opacity-75 font-extrabold text-3xl'}>
                                        {SwapModalData.MainButtonsText.InsufficientBalance}
                                    </GradientButton>
                                ) : (
                                    <div className='w-full'>
                                        {ClickedSwap ? (
                                            <div onClick={async () => {
                                                setApprovalSuccess()
                                                setSwapSuccess()
                                                openModalWithSteps()
                                                const res = await handleSwapApproval(PayCoin, backendCanisterID)
                                                setApprovalSuccess(res);
                                                if (res.success == true) {
                                                    const res = await swapHandler();
                                                    setSwapSuccess(res);
                                                }


                                            }}>
                                                {/* <Approval buttonText={'Confirm Swapping'}/> */}
                                                <GradientButton CustomCss={'w-full md:w-full font-extrabold text-3xl'} >
                                                    {SwapModalData.MainButtonsText.ConfirmSwapping}
                                                </GradientButton>
                                            </div>
                                        ) : (
                                            <div onClick={() => {
                                                setClickSwap(true);
                                            }}>
                                                <GradientButton CustomCss={'w-full md:w-full font-extrabold text-3xl'}>
                                                    {SwapModalData.MainButtonsText.SwapNow}
                                                </GradientButton>
                                            </div>
                                        )}
                                    </div>
                                )}
                            </div> : <GradientButton CustomCss={'w-full md:w-full cursor-auto disabled opacity-75 font-extrabold text-3xl'}>
                                Connect wallet
                            </GradientButton>}
                        </div>
                    )}
                </div>

                {/* {bothCoins && !ClickedSwap && (
                    <div className='w-full flex flex-col gap-4 p-4 mx-auto my-4 rounded-lg'>
                        <div className='flex justify-between'>
                            <div className='flex items-center gap-1 sm:gap-2'>
                                <span className='relative custom-text-size-14 sm:text-base'>
                                    {SwapModalData.ClickedSwapData.MinimumRecieved}
                                    {show1 && (
                                        <div className='z-50 absolute -ml-6 sm:ml-40 w-[250%]'>
                                            <DialogBox text={Message} />
                                        </div>
                                    )}
                                </span>
                                <span
                                    onMouseEnter={() => {
                                        setShow1(true);
                                        setMessage(SwapModalData.infoMessageMinimumRecieved);
                                    }}
                                    onMouseLeave={() => {
                                        setShow1(false);
                                    }}
                                >
                                    <Info size={20} />
                                </span>
                            </div>

                            <div className='font-gilroy font-medium text-base'>10.5580 CT</div>
                        </div>
                        <div className='flex justify-between'>
                            <div className='flex items-center gap-1 sm:gap-2'>
                                <span className='relative custom-text-size-14 sm:text-base'>
                                    {SwapModalData.ClickedSwapData.OverallSlippage}
                                    {show2 && (
                                        <div className='z-50 absolute -ml-6 sm:ml-40 w-[250%]'>
                                            <DialogBox text={Message} />
                                        </div>
                                    )}
                                </span>

                                <span
                                    onMouseEnter={() => {
                                        setShow2(true);
                                        setMessage(SwapModalData.infoMessageOverallSlippage);
                                    }}
                                    onMouseLeave={() => {
                                        setShow2(false);
                                    }}
                                >
                                    <Info size={20} />
                                </span>
                            </div>

                            <div><ImpactButton customCss={`font-bold`} Impact={'Positive'}>10%</ImpactButton></div>
                        </div>
                        <div className='flex justify-between gap-16 sm:gap-0 items-center'>
                            <div className='flex justify-evenly items-center gap-1 sm:gap-2 my-3'>
                                <span className='relative custom-text-size-14 sm:text-base'>
                                    {SwapModalData.ClickedSwapData.LiquidityProviderIncentive}
                                    {show3 && (
                                        <div className='z-50 mb-32 absolute sm:ml-40 w-[150%]'>
                                            <DialogBox text={Message} />
                                        </div>
                                    )}
                                </span>
                                <span
                                    className='cursor-pointer'
                                    onMouseEnter={() => {
                                        setShow3(true);
                                        setMessage(SwapModalData.infoMessageLiquidityProviderIncentive);
                                    }}
                                    onMouseLeave={() => {
                                        setShow3(false);
                                    }}
                                >
                                    <Info size={20} />
                                </span>
                            </div>

                            <div className='font-gilroy font-medium text-base'>0.000056 ETH</div>
                        </div>

                        <BorderGradientButton customCss={`w-full bg-[#000711] z-10`}>{SwapModalData.MainButtonsText.AnalysePair}</BorderGradientButton>
                    </div>
                )} */}
                
                {isModalOpen ? <div className="fixed inset-0  bg-black bg-opacity-50 flex justify-center items-center z-50">
                    <div className="p-6 pb-16 w-11/12 sm:max-w-[1200px] sm:mx-auto  border-2 border-[#86828280] bg-[#182030] mt-10 rounded-lg shadow-lg text-white  mx-auto relative">
                        <div className='ml-9 '>
                        <button
                            className="absolute top-5 right-10 text-gray-400 hover:text-gray-300"
                            onClick={() => setIsModalOpen(false)}
                        >
                            <CloseIcon/>
                        </button>

                        <h2 className="text-xl font-semibold mb-4 font-gilroy">Swap Details</h2>
                        <p className="text-gray-400 mb-6 font-gilroy">
                            You can swap directly without depositing, because you have sufficient balance in the Swap pool.
                        </p>
                        </div>

                        <div className='flex flex-col gap-y-6'>
                            <div className='flex gap-x-4 font-gilroy '>
                                <div className='flex justify-center items-center'>{approvalSuccess ? <CheckCircleOutlineIcon style={{ color: "green" }} /> : <CircularProgress size="20px" />}</div>
                                <div className='flex flex-col border rounded-lg  py-2 border-gray-600 bg-[#30303080]  w-full'>
                                    <div className='flex justify-between px-4 pb-1 w-full '>
                                        <div className='flex  '>
                                            <span>1. Approve <span>{PayCoin.ShortForm}</span></span>
                                            
                                        </div>
                                        <div onClick={() => setSubModel(1)}>
                                            {subModel == 1 ? <KeyboardArrowUpIcon /> : <KeyboardArrowDownIcon />}
                                        </div>
                                    </div>
                                    <div className={` ${subModel == 1 ? "flex flex-col" : 'hidden'}`} >
                                        <hr className=' border-gray-500' />
                                        <div className='flex gap-x-2 justify-between w-full font-extralight px-4 text-sm pt-1 text-[#FFFFFFBF]'>
                                            <span>Amount</span>
                                            <span className='flex gap-2 items-center'>
                                                <img src={PayCoin.ImagePath} alt="" className='w-4 h-4' />
                                                {CoinAmount}</span>
                                        </div>
                                        <div className='flex justify-between w-full font-extralight px-4 text-sm pb-1 text-[#FFFFFFBF]'>
                                            <span >Canister Id</span>
                                            <span>{PayCoin.CanisterId}</span>
                                        </div>
                                    </div>

                                </div>
                            </div>


                            <div className='flex gap-x-4 font-gilroy '>
                                <div className='flex justify-center items-center'>{approvalSuccess ? <CheckCircleOutlineIcon style={{ color: "green" }} /> : <CircularProgress size="20px" />}</div>
                                <div className='flex flex-col border rounded-lg  py-2 border-gray-600 bg-[#30303080]  w-full'>
                                    <div className='flex justify-between px-4 w-full '>
                                        <div className='flex  '>
                                            <span>1. Deposite <span>{PayCoin.ShortForm}</span></span>
                                            
                                        </div>
                                        <div onClick={() => setSubModel(2)}>
                                            {subModel == 2 ? <KeyboardArrowUpIcon /> : <KeyboardArrowDownIcon />}
                                        </div>
                                    </div>
                                    <div className={` ${subModel == 2 ? "flex flex-col" : 'hidden'}`} >
                                        <hr className=' border-gray-500' />
                                        <div className='flex gap-x-2 justify-between w-full font-extralight px-4 text-sm pt-1 text-[#FFFFFFBF]'>
                                            <span>Amount</span>
                                            <span className='flex gap-2 items-center'>
                                                <img src={PayCoin.ImagePath} alt="" className='w-4 h-4' />
                                                {CoinAmount}</span>
                                        </div>
                                        <div className='flex justify-between w-full font-extralight px-4 text-sm pb-1 text-[#FFFFFFBF]'>
                                            <span >Canister Id</span>
                                            <span>{PayCoin.CanisterId}</span>
                                        </div>
                                    </div>

                                </div>
                            </div>
                                      
                                      
                                          {/*  */}

                            <div className='flex gap-x-4 font-gilroy'>
                                <div className='flex justify-center items-center'>{swapSuccess ? <CheckCircleOutlineIcon style={{ color: "green" }} /> : <CircularProgress size="20px" />}</div>
                                <div className='flex flex-col border rounded-lg  py-2 border-gray-600 bg-[#30303080] w-full'>
                                    <div className='flex justify-between w-full px-4 '>
                                        <div className='flex gap-x-4'>
                                            <span>3. Swap <span>{PayCoin.ShortForm}</span> to <span>{RecieveCoin.ShortForm}</span></span>
                                          
                                        </div>
                                        <div onClick={() => setSubModel(3)}>
                                            {subModel == 3 ? <KeyboardArrowUpIcon /> : <KeyboardArrowDownIcon />}
                                        </div>
                                    </div>
                                    <div className={` ${subModel == 3 ? "flex flex-col" : 'hidden'}`}>
                                        <hr className=' border-gray-500' />
                                        <div className='flex gap-x-2 justify-between w-full font-extralight text-sm px-4 pt-1'>
                                            <span>{PayCoin.ShortForm}</span>
                                            <span className='flex gap-x-2 justify-center items-center'><img src={PayCoin.ImagePath} alt="" className='w-4 h-4' /> {CoinAmount}</span>
                                        </div>
                                        <div className='flex justify-between w-full font-extralight text-sm px-4 pb-1'>
                                            <span >{RecieveCoin.ShortForm}</span>
                                            <span className='flex gap-x-2 justify-center items-center'><img src={RecieveCoin.ImagePath} alt="" className='w-4 h-4' /> 44</span>
                                        </div>
                                    </div>

                                </div>
                            </div>


                            {/* withdraw */}
                            <div className='flex gap-x-4 font-gilroy'>
                                <div className='flex justify-center items-center'>{swapSuccess ? <CheckCircleOutlineIcon style={{ color: "green" }} /> : <CircularProgress size="20px" />}</div>
                                <div className='flex flex-col border rounded-lg  py-2 border-gray-600  bg-[#30303080] w-full'>
                                    <div className='flex justify-between px-4 w-full'>
                                        <div className='flex gap-x-4'>
                                            <span>3. Withdraw  <span>{RecieveCoin.ShortForm} </span></span>
                                       
                                        </div>
                                        <div onClick={() => setSubModel(4)}>
                                            {subModel == 4 ? <KeyboardArrowUpIcon /> : <KeyboardArrowDownIcon />}
                                        </div>
                                    </div>
                                    <div className={` ${subModel == 4 ? "flex flex-col" : 'hidden'}`}>
                                        <hr className=' border-gray-500' />

                                        <div className='flex justify-between w-full font-extralight text-sm px-4 pt-1 text-[#FFFFFFBF]'>
                                            <span >{RecieveCoin.ShortForm}</span>
                                            <span className='flex gap-x-2 justify-center items-center'><img src={RecieveCoin.ImagePath} alt="" className='w-4 h-4' /> 44</span>
                                        </div>
                                    </div>

                                </div>
                            </div>


                        </div>



                    </div>
                </div> : ""}
            </div>
        </div>
    );
};

export default Swap;
