import { useCallback, useMemo, useState } from "react";
import YourLiquidityPool from "./YourLiquidityPool";
import GradientButton from "../../buttons/GradientButton";
import WithdrawLiquidity from "./WithdrawLiquidity";
import TransactionComplete from "./TransactionComplete";
import BorderGradientButton from "../../buttons/BorderGradientButton";

const Container = () => {
    const [currView, setCurrView] = useState(0);

    const handleViewChange = () => {
        setCurrView((prev) => (prev + 1) % CURR_VIEW.length);
    };

    const CURR_VIEW = useMemo(()=>[
        {
            title: "Your Liquidity Pool Balance",
            component: <YourLiquidityPool />,
            CTA: (
                <GradientButton CustomCss="w-full" onClick={handleViewChange}>
                    Withdraw
                </GradientButton>
            ),
        },
        {
            title: "Withdraw Liquidity",
            component: <WithdrawLiquidity />,
            CTA: (
                <GradientButton CustomCss="w-full" onClick={handleViewChange}>
                    Confirm Withdraw
                </GradientButton>
            ),
        },
        {
            title: null,
            component: <TransactionComplete />,
            CTA: (
                <BorderGradientButton customCss={`bg-[#000711] z-10 !w-full`} onClick={()=>console.log("Close")}>
                    Close
                </BorderGradientButton>
            ),
        },
    ]);

    const renderView = useCallback(() => {
        return (
            <>
                {CURR_VIEW[currView].title && (
                    <>
                        <h2 className="md:text-2xl font-fahkwang text-center">{CURR_VIEW[currView].title}</h2>
                        <hr />
                    </>
                )}
                {CURR_VIEW[currView].component}
            </>
        );
    }, [currView]);

    return (
        <section
            className="md:min-w-[450px] min-w-[300px] aspect-square 
        bg-[#05071D] rounded-xl border border-['rgba(255, 255, 255, 0.5)']] 
        px-2 py-3 flex flex-col space-y-4"
        >
            {renderView()}
            {CURR_VIEW[currView].CTA}
        </section>
    );
};

export default Container;