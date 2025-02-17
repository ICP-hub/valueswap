import { searchCoinGeckoById } from "../components/utils/fetchCoinGeckoData";

export function formatToLocalConvention(bigIntValue, locale = 'en-IN') {
    if (typeof bigIntValue !== 'bigint') {
        throw new TypeError('Input must be a BigInt');
    }

    // Use Intl.NumberFormat for locale-specific formatting
    return new Intl.NumberFormat(locale).format(bigIntValue);
}

export const convertIntToCurrencyString = (value) => {
    const Numvalue = BigInt(value);
    let formattedValue;
    if (Numvalue >= 1_000_000_000) {
        formattedValue = `$${(Numvalue / 1_000_000_000n)}b`;
    } else if (Numvalue >= 1_000_000) {
        formattedValue = `$${parseFloat(Numvalue / 1_000_000n).toFixed(2)}m`;
    } else {
        formattedValue = `$${Numvalue.toLocaleString('en-US')}`;
    }
    return formattedValue;
}

let tokenListCache = null 

export const getTokenId = async (tokenSymbol) => {
    try {
        if (!tokenListCache) {
            const response = await fetch("https://api.coingecko.com/api/v3/coins/list");
            tokenListCache = await response.json(); // Ensure response is parsed as JSON
            console.log("Token list fetched:", tokenListCache); // Log the token list
        }

        const token = tokenListCache.find(
            (t) => t.symbol.toLowerCase() === tokenSymbol.toLowerCase()
        );

        console.log("Token found:", token); // Log the found token

        return token ? token.id : null;
    } catch (err) {
        console.error("Error fetching token list:", err);
        return null;
    }
};

// Use CoinGecko API to get the current price of the token
export const convertTokenEquivalentUSD = async (tokenName) => {
    try {
        console.log("Token name:", tokenName);
        const tokenID = await getTokenId(tokenName.toLowerCase());
        console.log("Token ID:", tokenID);
        if (!tokenID) {
            throw new Error('Token ID not found');
        }
        const token = await searchCoinGeckoById(tokenID);
        if (!token) {
            throw new Error('Token not found');
        }
        const currentPrice = token?.market_data?.current_price?.usd;
        return currentPrice;
    } catch (err) {
        console.error('Error fetching token data:', err);
    }
};