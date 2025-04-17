import { useEffect, useState } from "react";
import { usePoolData } from "../../hooks/usePoolData";

const TokenDisplay = ({ tokens, percentage, id, user_share_ratio }) => {
    const [displayToken, setDisplayToken] = useState(tokens);

    useEffect(() => {
        setDisplayToken(tokens);
    }, [tokens]);


    return (
        <div className="flex flex-col justify-center space-y-2">
            {displayToken.map((token, idx) => (
                <div
                    className="flex flex-row justify-between items-center w-full font-gilroy md:py-6 md:px-4 py-3 px-2 backdrop-blur-[32px] rounded-lg border-2 border-white border-opacity-50"
                    key={idx}
                >
                    <div className="flex flex-col gap-1">
                        <p className="md:text-5xl sm:text-3xl text-2xl">
                            {parseFloat(user_share_ratio[idx]) * (percentage * 0.01)}
                        </p>
                        <p className="tracking-widest">{parseFloat(token.weight)}%</p>
                    </div>
                    <div className="flex items-center space-x-2">
                        <span className="w-[40px] aspect-square rounded-full">
                            <img src={token.image} alt={token.token_name} />
                        </span>
                        <p className="md:text-2xl">{token.token_name}</p>
                    </div>
                </div>
            ))}
        </div>
    );
};

export default TokenDisplay;