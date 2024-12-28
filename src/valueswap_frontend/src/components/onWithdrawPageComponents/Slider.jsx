import { useState } from "react"
import BorderGradientButton from "../../buttons/BorderGradientButton"

const SLIDE_OPTIONS = [25, 50, 75, 100]

const Slider=()=>{
    const [value,setValue] = useState(SLIDE_OPTIONS[0])

    return(
        <div className="font-cabin flex flex-col space-y-4 backdrop-blur-[32px] ">
            <p>Select Amount</p>
            <div className="flex justify-between items-center space-x-4 mt-2">
            {
                SLIDE_OPTIONS.map((option,idx)=>(
                    <div onClick={()=>setValue(option)} key={idx}>
                    <button
      className="relative px-6 py-2 rounded-xl text-white font-medium
        bg-[#1a1b26]/40 backdrop-blur-[32px]
        before:absolute before:inset-0 before:rounded-xl before:bg-gradient-to-r before:from-cyan-500 before:to-blue-500 before:opacity-20 before:blur-xl
        after:absolute after:inset-0 after:rounded-xl after:bg-gradient-to-r after:from-cyan-500 after:to-blue-500 after:opacity-20 after:blur-xl
        hover:before:opacity-30 hover:after:opacity-30
        transition-all duration-300"
    >
      {option}%
    </button>
                    </div>
                ))
            }
            </div>
            <div className="w-full">
                <input type="range" value={value} onChange={(e)=>setValue(e.target.value)} min={25} step={25} max={100} name="value"
                className="accent-orange-500 w-full"
                />
            </div>
        </div>
    )
}

export default Slider