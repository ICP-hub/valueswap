
const TOKENS = [
    {
        token: {
            symbol: "CT",
            img: "",
            weights: 50,
            weightsLocked: false
        },
        unit: 45.9876
    },
    {
        token: {
            symbol: "LTC",
            img: "",
            weights: 50,
            weightsLocked: false
        },
        unit: 1.0256
    }
]
const TokenDisplay = () => {
    return (
        <div className="flex flex-col justify-center space-y-6 mt-2">
            {
                TOKENS.map((token, idx) => (
                    <div className="flex flex-row-reverse justify-between items-center w-full font-cabin">
                        <div className="flex items-center space-x-2">
                            <span className="bg-[#3E434B] py-1 rounded-lg px-1 md:px-3 relative inline-block">
                                <input
                                    type="number"
                                    className="bg-transparent w-5 focus:outline-none focus:cursor-n-resize text-xs hide-arrows"
                                    value={token.token.weights}
                                    disabled={true}
                                />
                                <span className='text-xs'>%</span>
                            </span>
                            <p className="tracking-widest">{token.unit.toPrecision(5)}</p>
                        </div>
                        <div className="flex items-center space-x-2">
                            <span className="w-[40px] aspect-square rounded-lg bg-[#3D3F47] border border-white">
                                <img src={token.token.img} alt={token.token.symbol} />
                            </span>
                            <p className="md:text-2xl">{token.token.symbol}</p>
                        </div>
                    </div>
                ))
            }
        </div>
    )
}

export default TokenDisplay