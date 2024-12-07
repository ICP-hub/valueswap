import React, { useState } from 'react'
import { X } from 'lucide-react'
import GradientButton from '../buttons/GradientButton'
const FaucetModal = ({setModelOpen, imgUrl, TokenName}) => {
  const [faucetAmount, setFaucetAmount] = useState(0)
   console.log("faucetAmount", faucetAmount)
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

const handleDepositeApprove = async (backendCanisterID) => {
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
        console.error('Error in handleDepositeApprove:', error);
        toast.error('Token approval failed. Please try again.');
        return { success: false, error: error.message };
    }
};

const withdrawCoinHandler = async () => {

    const scaledAmount = parseFloat(faucetAmount);
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
        handleDepositeApprove(CANISTER_ID_VALUESWAP_BACKEND).then(approveResult => {
            console.log("approve result", approveResult)

            const finallWidthdraw = async () => {
           
                const res = await backendActor?.deposit_tokens(scaledAmount, ledger_canister_id, target_canister_id);
                if (res.Err) {
                    toast.error("We are fixing the deposite issue");
                } else {
                    toast.success("Successfully deposite 🤞");
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

  console.log("TokenName", TokenName)
  return (
  <div className='fixed top-[12%] left-0 w-full h-full bg-opacity-50 backdrop-blur-sm'>
      <div className='  max-w-[480px] h-[300px] mt-36 mx-auto  bg-[#182030] rounded-lg  p-8'>
        <div className='flex justify-between'>
          <h1 className='text-2xl font-semibold'>
            Faucet ICP
          </h1>
          <X onClick={()=> setModelOpen(false)} className='cursor-pointer'/>
        </div>
        <div className='flex flex-col justify-center h-full gap-y-4'>
        <div>Transaction overview</div>
        <div className='flex justify-between'>
         <div className='my-auto'>
         <input type="number" className='bg-transparent p-2 border-2 border-[#3c3f44] active:border-[#3c3f44] focus:outline-none rounded-md' onChange={(e)=> setFaucetAmount(e.target.value)}/>
         </div>
          <div className='space-y-4'>
            <div className='flex gap-x-2 items-center'>
              <img src={imgUrl.imgUrl} alt="" className='w-10 h-10'/>
              <p className='font-medium text-lg'>{TokenName.TokenName}</p>
            </div>
            <div className='flex gap-x-1 bg-[#37415173] p-1 px-2 rounded-md'>
              <p className='text-gray-300'>Approx.</p>
              <p>$500 Max</p>
            </div>
          </div>
        </div>

          <GradientButton CustomCss={` w-full md:w-full`}>
         Faucet {TokenName.TokenName}
        </GradientButton>
        </div>
      </div>
      </div>

  )
}

export default FaucetModal