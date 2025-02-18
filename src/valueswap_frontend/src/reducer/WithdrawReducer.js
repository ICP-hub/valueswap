import { createSlice } from "@reduxjs/toolkit";

const Withdraw = createSlice({
    initialState : [],
    name: "withdraw",
    reducers: {
        setWithdraw: (state, action) => {
            state = action.payload;
        }
    }
})
export const {setWithdraw} = Withdraw.actions;

export default Withdraw.reducer;