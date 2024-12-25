import { useCallback, useState } from "react"
import YourLiquidityPool from "./YourLiquidityPool"
import GradientButton from "../../buttons/GradientButton"
import WithdrawLiquidity from "./WithdrawLiquidity"

const CURR_VIEW = [
    {
        title : "Your Liquidity Pool Balance",
        component : <YourLiquidityPool/>
    },
    {
        title : "Withdraw Liquidity",
        component : <WithdrawLiquidity/>
    },
    {
        title : null,
        component : <></>
    }
]

const Container = ()=>{
    const [currView, setCurrView] = useState(0)

    const renderView = useCallback(()=>{
        return(
            <>
                {CURR_VIEW[currView].title && (
                    <>
                    <h2 className="md:text-2xl font-fahkwang text-center">{CURR_VIEW[currView].title}</h2>
                    <hr />
                    </>
                )}
                {CURR_VIEW[currView].component}
            </>
        )
    },[currView])

    const handleViewChange=()=>{
        setCurrView((prev)=>((prev+1) % CURR_VIEW.length))
    }

    return(
        <section className="md:min-w-[450px] min-w-[300px] aspect-square 
        bg-[#05071D] rounded-xl border border-['rgba(255, 255, 255, 0.5)']] 
        px-2 py-3 flex flex-col space-y-4">
            {renderView()}
            <GradientButton CustomCss="w-full" onClick={handleViewChange}>
                {currView === 0 ? "Withdraw" : "Confirm Withdraw"}
            </GradientButton>
        </section>
    )
}

export default Container