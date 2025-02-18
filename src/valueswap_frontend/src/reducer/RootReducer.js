import alertReducer from "./Alert";
import PoolCreation from "./PoolCreation";
import walletReducer from "./WalletSlice";
import withdrawReducer from "./WithdrawReducer";
import { combineReducers } from '@reduxjs/toolkit';

const rootReducer = combineReducers({
    alert: alertReducer,
    pool: PoolCreation,
    wallet: walletReducer,
    withdraw : withdrawReducer
});

export default rootReducer;