import LiquidityRatio from "./LiquidityRatio"
import Slider from "./Slider"
import TokenDisplay from "./TokenDisplay"
import WETHConsent from "./WETHConsent"

const WithdrawLiquidity=()=>{
    return(
        <div className="flex flex-col space-y-2">
            <LiquidityRatio/>
            <Slider/>
            <TokenDisplay/>
            <WETHConsent/>
        </div>
    )
}

export default WithdrawLiquidity