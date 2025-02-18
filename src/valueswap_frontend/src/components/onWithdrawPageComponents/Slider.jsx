import { useCallback, useEffect, useState } from "react"
import BorderGradientButton from "../../buttons/BorderGradientButton"
import { useAuthClient } from "../utils/useAuthClient"

const SLIDE_OPTIONS = [25, 50, 75, 100]

const Slider = ({poolData}) => {
    const [value, setValue] = useState(SLIDE_OPTIONS[0])
    const {backendActor} = useAuthClient()

    const getUserShareRatio = useCallback(async()=>{
        console.log("poolData", poolData)
        try{
            const response = await backendActor.get_user_share_ratio(poolData[0],"ckbtccketh", 1234)
            console.log("response", response)
        }catch(err){    
            console.error("Error getting user share ratio", err)
        }finally{
            console.log("done getting user share ratio")
        }
    },[poolData, backendActor])

    useEffect(()=>{
        getUserShareRatio()
    },[value])

    return (
        <div className="font-cabin flex flex-col space-y-4 backdrop-blur-[32px]">
            <p>Select Amount</p>
            <div className="flex justify-between items-center space-x-4 mt-2">
                {
                    SLIDE_OPTIONS.map((option, idx) => (
                            <button
                                onClick={() => setValue(option)}
                                aria-pressed={value === option}
                                type="button"
                                key={option}
                                className="group relative flex md:h-10 md:w-24 h-5 w-12 items-center justify-center focus:outline-none"
                            >
                                <div className="absolute -inset-0.5 rounded-lg bg-gradient-to-r from-blue-100/20 to-white/20 opacity-0 blur-md group-aria-pressed:opacity-100 transition-opacity duration-300 ease-in" />
                                <div className="relative flex h-full w-full items-center justify-center rounded-lg bg-gray-900 ring-1 ring-gray-700/50">
                                    {option === 100 ? (
                                        <span className="text-sm font-medium text-white">MAX</span>
                                    ):
                                    (
                                        <span className="text-sm font-medium text-white">{option}</span>
                                    )}
                                </div>
                            </button>
                    ))
                }
            </div>
            <div className="w-full">
                <input type="range" value={value} onChange={(e) => setValue(e.target.value)} min={25} step={25} max={100} name="value"
                    className="accent-orange-500 w-full"
                />
            </div>
        </div>
    )
}

export default Slider