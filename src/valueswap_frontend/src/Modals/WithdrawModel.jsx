import React, { useState } from 'react';
import GradientButton from '../buttons/GradientButton';
import { useAuth } from '../components/utils/useAuthClient';
import { toast } from 'react-toastify';
import CloseOutlinedIcon from '@mui/icons-material/CloseOutlined';
import CircularProgress from '@mui/material/CircularProgress';
import { idlFactory as tokenIdl } from '../../../declarations/ckbtc_ledger';
import { Principal } from '@dfinity/principal';
function WithdrawModel({ setOpenWithdraw, poolName }) {
    const [change, setChange] = useState(false);
    const [coinDetail, setCoinDetail] = useState([]);
    const [CoinName, setCoinName] = useState([])
    // const { backendActor } = useAuth();
    const [amountLp, setAmountLp] = useState(0);
    const { createTokenActor, backendActor, principal, getBalance } = useAuth();


    const fetchCoinFromLp = () => { 
        setChange(true);

        return new Promise((resolve, reject) => {
            backendActor?.get_specific_pool_data(poolName)
                .then(specificData => {
                    let pool_data = specificData.Ok[0].pool_data;
                    let swap_fee = specificData.Ok[0].swap_fee;
                    setCoinName(pool_data)
                    console.log("pooldata:", pool_data);
                    console.log("swap_fee:", swap_fee);

                    if (pool_data) {
                        console.log("calling start");
                        return backendActor?.get_user_share_ratio(
                            { pool_data: pool_data, swap_fee: swap_fee },
                            poolName,
                            parseFloat(amountLp) * Math.pow(10, 8)
                        );
                    } else {
                        reject(new Error("Invalid pool data."));
                    }
                })
                .then(res => {
                    console.log("res", res);
                    if (res.error) {
                        toast.error("We are fixing the coin issue");
                        reject(new Error("Coin issue encountered."));
                    } else {
                        setCoinDetail(res.Ok);

                        resolve(res);
                    }
                })
                .catch(error => {
                    console.error("Error fetching coin from LP:", error);
                    toast.error("An error occurred while fetching coin details.");
                    reject(error);
                });
        });
    };


    console.log("getBalance", getBalance(process.env.CANISTER_ID_LP_LEDGER_CANISTER));

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


    const handleCreatePoolClick = async (backendCanisterID) => {
        const CanisterId = process.env.CANISTER_ID_LP_LEDGER_CANISTER;
        try {
            if (!CanisterId || !amountLp) {
                console.error("Invalid CanisterId or amount");
                return { success: false, error: "Invalid CanisterId or amount" };
            }
            const tokenActor = await createTokenActor(CanisterId);
            const approvalTransactions = await transferApprove(
                amountLp,
                CanisterId,
                backendCanisterID,
                tokenActor
            );

            // Execute batch transactions
            const result = await window.ic.plug.batchTransactions(approvalTransactions);

        
            return { success: true, data: result };
        } catch (error) {
            console.error('Error in handleCreatePoolClick:', error);
            toast.error('Token approval failed. Please try again.');
            return { success: false, error: error.message };
        }
    };


    const withdrawCoinHandler = async () => {

        const scaledAmount = parseFloat(amountLp);
        if (isNaN(scaledAmount) || scaledAmount <= 0) {
            toast.error("Invalid amount. Please enter a valid number.");
            return;
        }
        console.log("scaledAmount", typeof (scaledAmount))
        if (scaledAmount <= 0) {
            toast.error("Please enter a valid amount to withdraw.");
            return;
        }
        const CANISTER_ID_VALUESWAP_BACKEND = process.env.CANISTER_ID_VALUESWAP_BACKEND;
        try {
            handleCreatePoolClick(CANISTER_ID_VALUESWAP_BACKEND).then(approveResult => {
                console.log("approve result", approveResult)

                const finallWidthdraw = async () => {
                    const specificData = await backendActor?.get_specific_pool_data(poolName);
                    let pool_data = await specificData.Ok[0].pool_data;
                    let swap_fee = await specificData.Ok[0].swap_fee;
                    const res = await backendActor.burn_lp_tokens({ pool_data: pool_data, swap_fee: swap_fee }, poolName, scaledAmount);
                    if (res.Err) {
                        toast.error("We are fixing the withdraw issue");
                    } else {
                        toast.success("Successfully withdrew 🤞");
                        setChange(false);
                        return res;
                    }
                }
                if (approveResult.success) {

                    return finallWidthdraw()
                }
            })

        } catch (error) {
            console.error("Error during withdrawal:", error);
            toast.error("An error occurred while processing your request.");
        }
    };

    return (
        <div className='fixed inset-0 z-50 flex justify-center items-center bg-black bg-opacity-50 font-gilroy'>
            <div className='relative w-10/12 md:w-2/5 lg:w-1/3 bg-[#010427] px-2 pt-4 h-5/6 sm:h-4/6 border-2 rounded-lg z-50'>
                <div className='w-full flex justify-end cursor-pointer pr-4 z-50'>
                    <button onClick={() => setOpenWithdraw(false)}>
                        <CloseOutlinedIcon />
                    </button>
                </div>
                <div className='absolute top-[14%] bottom-0 left-0 right-0 m-auto z-10 flex flex-col gap-y-4 px-8'>
                    <label htmlFor="number" className='text-lg font-gilroy font-medium'>Input your LP</label>
                    <ul className='text-sm font-light list-disc list-inside'>
                        <li>First it shows your coin relative to the LP you input.</li>
                        <li>Then you can withdraw your coin.</li>
                    </ul>
                    <input
                        type="number"
                        name="lp"
                        id=""
                        className='rounded-2xl bg-[#1e2021a1] p-2'
                        min="0"
                        defaultValue={0}
                        // value={amountLp}
                        onChange={(e) => setAmountLp(Number(e.target.value))}
                    />
                    <div className='flex justify-center' onClick={() => change ? withdrawCoinHandler() : fetchCoinFromLp()}>
                        <GradientButton CustomCss={`text-xs md:text-base lg:text-base lg:w-[150px] py-2 px-2`}>
                            {change ? "Withdraw" : "Fetch Detail"}
                        </GradientButton>
                    </div>
                    <div>
                        {/* Display coin details */}
                        {CoinName.map((coin, index) => (
                            <div key={index} className='flex justify-between'>
                                <span>{coin.token_name}</span>
                                {coinDetail[index] ? (
                                    coinDetail[index] >= 0.00001 ? (
                                        <span>{(coinDetail[index]).toFixed(8)}</span>
                                    ) : (
                                        <span>Insufficient Balance</span>
                                    )
                                ) : (
                                    <CircularProgress color="secondary" size={20} />
                                )}



                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default WithdrawModel;