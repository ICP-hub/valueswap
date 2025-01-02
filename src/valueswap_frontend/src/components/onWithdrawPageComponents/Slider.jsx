import { useState } from "react"
import BorderGradientButton from "../../buttons/BorderGradientButton"

const SLIDE_OPTIONS = [25, 50, 75, 100]

const Slider = () => {
    const [value, setValue] = useState(SLIDE_OPTIONS[0])

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