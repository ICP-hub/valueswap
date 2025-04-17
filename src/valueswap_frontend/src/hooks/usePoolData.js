import { useCallback, useEffect, useState } from "react"
import {useAuths} from "../components/utils/useAuthClient"
import { Principal } from "@dfinity/principal";

export const usePoolData = (id) => {
    const [tokens, setTokens] = useState([]);
    const [poolData, setPoolData] = useState([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);
    const [user_share_ratio , _set_user_share] = useState([0,0])
    const [user_pool_lp, _set_user_pool_lp] = useState(0)
    const [percentage, _set_percentage] = useState(25)

    const {backendActor, principal, createTokenActor} = useAuths()

    const getPoolData = useCallback(async()=>{
      setLoading(true)
        try{
          const data = await backendActor.get_specific_pool_data(id)
          if(data?.Ok){
            console.log("pool data", data.Ok)
            const pool_datas = data.Ok 
            setPoolData(pool_datas)
            setTokens(pool_datas[0].pool_data)
          }else{
            throw new Error(data.Err)
          }
        }catch(err){
          console.error("Error fetching pool data", err)
          setError(err)
          setTokens([])
        }finally{
          console.log("done fetching pool data",tokens)
          setLoading(false)
        }
      },[id])

      console.log("Percent : ", percentage)
      console.log("USER POOL LP : ", user_pool_lp)

      const getUserPoolWithLP = useCallback(async()=>{
        const USER_PRINCIPAL = Principal.fromText(principal)
        console.log("Principal : ", USER_PRINCIPAL)
        try{
          const response = await backendActor.get_user_pools_with_lp(USER_PRINCIPAL)
          // Validate
          if(response.length === 0){
            throw new Error("No pool data found")
          }else{
            const pool = response.find((pool) => {
              console.log("pool id from : ", pool[0][0])
              return pool[0][0] === id
            })
            if(pool){
              console.log("pool ", pool[0][1])
              _set_user_pool_lp(pool[0][1])
            }else{
              throw new Error("No pool data found for current Pool ", id)
            }
          }
        }catch(err){
          console.error(err)
        }
      },[principal])

      const getUserShareRatio = useCallback(async()=>{
        console.log("poolData", {
            poolData : poolData[0],
            name : id,
            amount : user_pool_lp
        })
        try{
            const response = await backendActor.get_user_share_ratio(poolData[0],id, user_pool_lp)
            if(response?.Ok){
              console.log("SU : ", response.Ok)
              _set_user_share(response.Ok)
            }else{
              throw new Error(response.err)
            }
        }catch(err){    
            console.error("Error getting user share ratio", err)
        }
    },[poolData, backendActor,user_pool_lp,id])

    const handlePercentageChange = useCallback((value) => {
      console.log("Percentage changed to: ", value)
        _set_percentage(value);
    }, []);

    const createLPActor = async()=>{
      const LP_LEDGER_PRINCIPAL = Principal.fromText(process.env.CANISTER_ID_LP_LEDGER_CANISTER)
      const LP_ACTOR = await createTokenActor(LP_LEDGER_PRINCIPAL)
      return LP_ACTOR

    }

    const handleApprove = async()=>{
      const LP_ACTOR = await createLPActor()

      const amount_1 = parseFloat(user_pool_lp)

      const transaction = {
        amount: BigInt(amount_1),
        from_subaccount: [],
        spender: {
          owner: Principal.fromText(process.env.CANISTER_ID_VALUESWAP_BACKEND),
          subaccount: [],
        },
        fee: [],
        memo: [],
        created_at_time: [],
        expected_allowance: [],
        expires_at: [],
      };

      const approve = await LP_ACTOR.icrc2_approve(transaction)

      console.log("approve", approve)
    }

    const withdrawLiquidity = async()=>{
      const approval = await handleApprove()

      const LP_LEDGER_PRINCIPAL = Principal.fromText(process.env.CANISTER_ID_LP_LEDGER_CANISTER)
      const amount_to_burn = BigInt(parseFloat(user_pool_lp) * parseFloat(percentage / 100))
      console.log("Approval successful ", amount_to_burn, user_pool_lp)
      const burn_response = await backendActor.burn_lp_tokens(
        poolData[0],
        id,
        amount_to_burn,
        LP_LEDGER_PRINCIPAL
      )

      console.log("Burn response", burn_response)
      if(burn_response?.Ok){
        console.log("Burn successful")
      }
      else{
        console.error("Burn failed", burn_response)
      }
    }

    useEffect(()=>{
      getPoolData()
      getUserPoolWithLP()
    },[id, backendActor, principal, getUserPoolWithLP, getPoolData])

    useEffect(()=>{
      getUserShareRatio()
    },[percentage, getUserShareRatio, user_pool_lp])

  return { getPoolData, getUserPoolWithLP, tokens, poolData, loading, error, user_share_ratio, getUserShareRatio, handlePercentageChange, percentage, user_pool_lp, withdrawLiquidity }
}