import { useState } from "react"
import BorderGradientButton from "../../buttons/BorderGradientButton"

const SLIDE_OPTIONS = [25, 50, 75, 100]

const Slider=()=>{
    const [value,setValue] = useState(25)

    return(
        <div className="font-cabin flex flex-col space-y-4">
            <span className="w-full flex justify-between items-center">
                <p>Select Amount</p>
                <p className="md:text-4xl">{value}%</p>
            </span>
            <div className="flex justify-between items-center space-x-4 mt-2">
            {
                SLIDE_OPTIONS.map((option,idx)=>(
                    <div onClick={()=>setValue(option)} key={idx}>
                    <BorderGradientButton customCss={`bg-[#000711] z-10 !px-6`}>
                        {option}
                    </BorderGradientButton>
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