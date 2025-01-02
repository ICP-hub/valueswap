import { useState } from "react"

const TokenCard = ({ idx,token, unit }) => {

    const [tokenWeights, setTokenWeights] = useState(token.weights)

    const handleChangePercent = (e) => {
        setTokenWeights(e.target.value)
    }

    return(
        <div className="flex justify-between items-center w-full font-cabin">
            <div className="flex items-center space-x-2">
                <p>Token {idx}</p>
                <span className="bg-[#3E434B] py-1 rounded-lg px-1 md:px-3 relative inline-block">
                    <input
                        type="number"
                        className="bg-transparent w-5 focus:outline-none focus:cursor-n-resize text-xs hide-arrows"
                        value={tokenWeights}
                        onChange={handleChangePercent}
                        disabled={token.weightsLocked}
                    />
                    <span className='text-xs'>%</span>
                </span>
                <p className="tracking-widest">{unit.toPrecision(5)}</p>
            </div>
            <div className="flex items-center space-x-2">
                <span className="w-[40px] aspect-square rounded-lg bg-[#3D3F47] border border-white">
                    <img src={token.img} alt={token.symbol} />
                </span>
                <p className="md:text-2xl">{token.symbol}</p>
            </div>
        </div>
    )
}

export default TokenCard