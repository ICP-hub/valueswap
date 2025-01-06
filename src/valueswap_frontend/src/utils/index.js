
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