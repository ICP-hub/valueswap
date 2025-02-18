import TokenCard from "./TokenCard" 

const TOKENS = [
    {
        token : {
            symbol : "CT",
            img : "",
            weights : 50,
            weightsLocked : false
        },
        unit : 45.9876
    },
    {
        token : {
            symbol : "LTC",
            img : "",
            weights : 50,
            weightsLocked : false
        },
        unit : 1.0256
    }
]

const YourLiquidityPool = () => {
    return(
        <div className="flex flex-col justify-center space-y-8">
            <h3 className="font-cabin text-center lg:text-5xl">$189.05</h3>
            <ul className="even:divide-y space-y-2
             divide-['rgba(255, 255, 255, 0.5)']">
            {
                TOKENS.map((token,idx)=>(
                    <li key={idx} className="px-2 py-4">
                        <TokenCard idx={idx+1} {...token}/>
                    </li>
                ))
            }
            </ul>
        </div>
    )
}

export default YourLiquidityPool    