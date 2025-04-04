import { useCallback, useEffect, useState } from "react"
import {useAuths} from "../components/utils/useAuthClient"

export const usePoolData = (id) => {
    const [tokens, setTokens] = useState([]);
    const [poolData, setPoolData] = useState([]);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    const {backendActor, principal} = useAuths()

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

      useEffect(()=>{
        getPoolData()
      },[id, backendActor])

    return {tokens, poolData, loading, error}
}