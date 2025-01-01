import { useState, useCallback } from 'react';
import { useAuth } from '../components/utils/useAuthClient';
import { toast } from 'react-toastify';
import { Principal } from '@dfinity/principal';

export const useWithdraw = () => {
  const [change, setChange] = useState(false);
  const [coinDetail, setCoinDetail] = useState([]);
  const [CoinName, setCoinName] = useState([]);
  const [amountLp, setAmountLp] = useState(0);
  const { createTokenActor, backendActor, principal, getBalance } = useAuth();

  const fetchCoinFromLp = useCallback((poolName) => {
    setChange(true);

    return new Promise((resolve, reject) => {
      backendActor?.get_specific_pool_data(poolName)
        .then(specificData => {
          let pool_data = specificData.Ok[0].pool_data;
          let swap_fee = specificData.Ok[0].swap_fee;
          setCoinName(pool_data);

          if (pool_data) {
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
          if (res.error) {
            toast.error("We are fixing the coin issue");
            reject(new Error("Coin issue encountered."));
          } else {
            setCoinDetail(res.Ok);
            resolve(res);
          }
        })
        .catch(error => {
          toast.error("An error occurred while fetching coin details.");
          reject(error);
        });
    });
  }, [amountLp, backendActor]);

  const transferApprove = useCallback(async (sendAmount, canisterId, backendCanisterID, tokenActor) => {
    try {
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
      amount = parseInt(Number(sendAmount) * Math.pow(10, decimals));
      balance = await getBalance(canisterId);

      if (balance >= amount + fee) {
        const transaction = {
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
        };

        const response = await tokenActor.icrc2_approve(transaction);

        if (response?.Err) {
          toast.error("approve failed");
          return { success: false, error: response.Err };
        } else {
          toast.success("approve success");
          return { success: true, data: response.Ok };
        }
      } else {
        return { success: false, error: "Insufficient balance" };
      }
    } catch (error) {
      toast.error("approve failed");
      return { success: false, error: error.message };
    }
  }, [getBalance]);

  return {
    change,
    coinDetail,
    CoinName,
    amountLp,
    setAmountLp,
    fetchCoinFromLp,
    transferApprove,
  };
};