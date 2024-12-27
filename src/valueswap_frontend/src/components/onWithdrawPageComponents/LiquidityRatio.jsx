const TOKENS = [
    {
        token:{
            symbol:"CT",
            img:"",
            percentage:80
        }
    },
    {
        token:{
            symbol:"LTC",
            img:"",
            percentage:20
        }
    }
]

const LiquidityRatio=()=>{
    return(
        <div className="flex justify-start items-center w-full font-cabin md:space-x-4 lg:space-x-6">
            <div className="inline-flex items-center space-x-2">
                <span className="w-[40px] aspect-square rounded-lg bg-[#3D3F47] border border-white">
                    <img src={TOKENS[0].token.img} alt={TOKENS[0].token.symbol} />
                </span>
                <span className="w-[40px] aspect-square rounded-lg bg-[#3D3F47] border border-white">
                <img src={TOKENS[1].token.img} alt={TOKENS[1].token.symbol} />
                </span>
            </div>
            <p className="md:text-2xl tracking-widest">{TOKENS[0].token.symbol}/{TOKENS[1].token.symbol}::{TOKENS[0].token.percentage}/{TOKENS[1].token.percentage}</p>
        </div>
    )
}

export default LiquidityRatio