import { createSlice } from "@reduxjs/toolkit";

const initialState = {
    CoinCount: 2,
    FeeShare: 0,
    PercentShare: 50,
    TotalAmount: 0,
    Confirmation: false,
    TotalPercentage: 100,
    TotalAmount: 0,
    Tokens: [
        {
            id: "",
            Name: 'Token1',
            ShortForm: 'Token 1',
            Amount: 0,
            Selected: false,
            weights: 50,
            ImagePath: null,
            Amount: 0.0,
            marketPrice: 0,
            currencyAmount: 0,
            weightsLocked: false,
            CanisterId: null,
            metaData: 0,
        },
        {
            id: "",
            Name: "Token2",
            ShortForm: 'Token 2',
            Amount: 0,
            Selected: false,
            weights: 50,
            ImagePath: null,
            Amount: 0,
            marketPrice: 0,
            currencyAmount: 0,
            weightsLocked: false,
            CanisterId: null,
            metaData: 0,
        }
    ],

}

const Pool = createSlice({
    name: "pool",
    initialState,
    reducers: {
        AddCoin: (state, action) => {
            state.CoinCount += 1;
            let coinCount = state.CoinCount;
            state.Tokens.forEach((token) => {
                if (token.weightsLocked) {
                    coinCount -= 1;
                }
            });
            let PercentShare = parseFloat(state.TotalPercentage / coinCount).toFixed(2)
            state.Tokens.push(
                {
                    id: "id",
                    Name: 'new Token',
                    ShortForm: `Token ${state.CoinCount}`,
                    Amount: 0,
                    Selected: false,
                    weights: PercentShare,
                    ImagePath: null,
                    marketPrice: 0,
                    currencyAmount: 0,
                    weightsLocked: false,
                    metaData: 0,
                }
            )
            state.Tokens.forEach((token) => {
                if (!token.weightsLocked) {
                    token.weights = PercentShare;
                }
            });

            state.TotalAmount = SumUpValue(state.Tokens)
        },
        RemoveCoin: (state, action) => {
            const removedIndex = action.payload.index;
            state.CoinCount -= 1;
            let coinCount = state.CoinCount;
            state.Tokens.forEach((token) => {
                if (token.weightsLocked) {
                    coinCount -= 1;
                }
            });


            let TempToken = state.Tokens
            TempToken.splice(removedIndex, 1);

            state.Tokens = TempToken
            const newPercentShare = parseFloat(state.TotalPercentage / coinCount).toFixed(2);
            state.Tokens.forEach((token) => {
                if (!token.weightsLocked) {
                    token.weights = newPercentShare;
                }
            });
            // const lastCoinIndex = state.Tokens.length - 1;
            // state.Tokens[lastCoinIndex].weights = (state.TotalPercentage - newPercentShare * (state.CoinCount - 1)).toFixed(2);
            state.TotalAmount = SumUpValue(state.Tokens)

        },
        setWeightedPercent: (state, action) => {
            const index = action.payload.index;
            const newPercent = parseFloat(action.payload.percent);
            const oldPercent = parseFloat(state.Tokens[index].weights);
            const difference = newPercent - oldPercent;
            state.Tokens[index].weights = newPercent;

            // Adjust other tokens' weights to maintain the total percentage
            const unlockedTokens = state.Tokens.filter((token, i) => i !== index && !token.weightsLocked);
            const totalUnlockedWeights = unlockedTokens.reduce((total, token) => total + parseFloat(token.weights), 0);

            unlockedTokens.forEach((token) => {
                token.weights = parseFloat(token.weights) - parseFloat((difference * (parseFloat(token.weights) / totalUnlockedWeights)).toFixed(2));
            });

            // Ensure the sum of weights is exactly equal to TotalPercentage
            const totalWeights = state.Tokens.reduce((total, token) => total + parseFloat(token.weights), 0);
            const adjustment = state.TotalPercentage - totalWeights;
            if (adjustment !== 0) {
                const lastUnlockedToken = unlockedTokens[unlockedTokens.length - 1];
                if (lastUnlockedToken) {
                    lastUnlockedToken.weights = parseFloat(lastUnlockedToken.weights) + adjustment;
                }
            }
        },
        ToggleLocked: (state, action) => {
            const index = action.payload.index;
            state.Tokens[index].weightsLocked = action.payload.toggle

            if (action.payload.toggle === true) {
                console.log("percent gya", action.payload.percent)
                state.TotalPercentage -= parseFloat(action.payload.percent)
            } else {
                console.log("percent waapis aaya", typeof action.payload.percent)
                state.TotalPercentage += parseFloat(action.payload.percent)
            }

            console.log("new total percentage", state.TotalPercentage)
        },
        SetToken: (state, action) => {

            const index = action.payload.index
            state.Tokens[index].id = action.payload.TokenData.id;
            state.Tokens[index].Name = action.payload.TokenData.Name;
            state.Tokens[index].ShortForm = action.payload.TokenData.ShortForm;
            state.Tokens[index].ImagePath = action.payload.TokenData.ImagePath;
            state.Tokens[index].CanisterId = action.payload.TokenData.CanisterId;
            state.Tokens[index]. marketPrice = action.payload.TokenData.marketPrice;
            state.Tokens[index].currencyAmount = action.payload.TokenData.currencyAmount;
            state.Tokens[index].Selected = true;
            state.Tokens[index].metaData = action.payload.TokenData.metaData;
            state.TotalAmount = SumUpValue(state.Tokens)
        },
        SetFeeShare: (state, action) => {
            console.log("fee share to set:->", action)
            state.FeeShare = action.payload.FeeShare;
        },
        UpdateAmount: (state, action) => {
            console.log("amount update reducer called", action)
            const index = action.payload.index;
            state.Tokens[index].Amount = action.payload.Amount
            state.Tokens[index].currencyAmount = parseFloat((state.Tokens[index].Amount * state.Tokens[index].marketPrice).toFixed(3)),
            state.TotalAmount = state.Tokens.reduce((total, token) => total + token.Amount, 0);
            state.TotalAmount = SumUpValue(state.Tokens)
        },
        toggleConfirm: (state, action) => {
            // console.log(" Reducer called in page", action.payload.page)
            state.Confirmation = action.payload.value;
        }

    }
});

export const { AddCoin, RemoveCoin, SetToken, SetFeeShare, UpdateAmount, toggleConfirm, ToggleLocked, setWeightedPercent } = Pool.actions;
export default Pool.reducer;
export const SumUpValue = (tokens) => {
    return tokens.reduce((total, token) => total + token.currencyAmount, 0);
};