
const TOKENS = [
    {
        token: {
            symbol: "BTC",
            img: "/image/ckBTC.svg",
            weights: 50,
            weightsLocked: false
        },
        unit: 45.9876
    },
    {
        token: {
            symbol: "ETH",
            img: "/image/ckETH.svg",
            weights: 50,
            weightsLocked: false
        },
        unit: 1.0256
    }
]
const TokenDisplay = () => {
    return (
        <div className="flex flex-col justify-center space-y-2">
            {
                TOKENS.map((token, idx) => (
                    <div className="flex flex-row justify-between items-center w-full font-gilroy md:py-6 md:px-4 py-3 px-2 backdrop-blur-[32px] rounded-lg border-2 border-white border-opacity-50">
                        <div className="flex flex-col gap-1">
                               <p className="md:text-5xl sm:text-3xl text-2xl">{token.unit.toPrecision(5)}</p>
                                <p className="tracking-widest">{token.token.weights}%</p>
                        </div>
                        <div className="flex items-center space-x-2">
                            <span className="w-[40px] aspect-square rounded-full">
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