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

// Use CoinGecko API to get the current price of the token
export const convertTokenEquivalentUSD = async(tokenName, tokenPrice) => {
    try {
        const token = await searchCoinGeckoById(tokenName.toLowerCase());
        if (!token) {
            throw new Error('Token not found');
        }
        const currentPrice = token?.market_data?.current_price?.usd
        return currentPrice * tokenPrice;
    } catch (err) {
        console.error('Error fetching token data:', err);
    }
}