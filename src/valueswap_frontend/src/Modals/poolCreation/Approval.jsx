import React, { useState } from 'react'
import { useAuths } from '../../components/utils/useAuthClient';
import GradientButton from '../../buttons/GradientButton';
import { useSelector } from 'react-redux';
import { Principal } from '@dfinity/principal';

function Approval({buttonText, handleCreatePoolClick, setPoolCreated, setConfirmPool}) {
    const [poolData, setPoolData] = useState(null)
    const { backendActor, isAuthenticated } = useAuths();
    const { Tokens, Confirmation, TotalAmount, FeeShare } = useSelector((state) => state.pool);
    const [selectedTokenDetails, setSelectedTokenDetails] = useState();
    const createPoolHandler = () => {
      console.log("You clicked to create pool");
  
      // Check if Tokens array is valid
      if (!Tokens || Tokens.length === 0) {
        const errorMsg = "No tokens available to create a pool";
        console.error(errorMsg);
        return Promise.reject(new Error(errorMsg));
      }
  
      // Process each token and map to pool_data
      const pool_data = Tokens.map((token, index) => {
        return new Promise((resolve) => {
          console.log(`Processing token at index ${index}:`, token);
  
          // Check if token properties exist
          if (
            token.weights === undefined ||
            token.Amount === undefined ||
            token.currencyAmount === undefined ||
            token.CanisterId === undefined
          ) {
            const errorMsg = `Missing data for token ${token.ShortForm}`;
            console.error(errorMsg);
            return resolve(null); // Resolve with null to indicate invalid token
          }
  
          // Validate and parse weight
          const weightStr = token.weights.toString().replace(/[^0-9.-]+/g, "");
          const weight = parseFloat(weightStr);
          console.log(`Token ${token.ShortForm} weightStr:`, weightStr);
          console.log(`Token ${token.ShortForm} parsed weight:`, weight);
  
          if (!Number.isFinite(weight)) {
            const errorMsg = `Invalid weight value for token ${token.ShortForm}: ${token.weights}`;
            console.error(errorMsg);
            return resolve(null);
          }
          const normalizedWeight = weight / 100;
          console.log(`Token ${token.ShortForm} normalized weight:`, normalizedWeight);
  
          // Validate and parse amount
          const amountStr = token.Amount.toString().replace(/[^0-9.-]+/g, "");
          const amount = parseFloat(amountStr);
          console.log(`Token ${token.ShortForm} amountStr:`, amountStr);
          console.log(`Token ${token.ShortForm} parsed amount:`, amount);
  
          if (!Number.isFinite(amount)) {
            const errorMsg = `Invalid amount value for token ${token.ShortForm}: ${token.Amount}`;
            console.error(errorMsg);
            return resolve(null);
          }
          const balance = BigInt(Math.round(amount));
          console.log(`Token ${token.ShortForm} balance (BigInt):`, balance);
  
          // Validate and parse currency amount
          const currencyAmountStr = token.currencyAmount.toString().replace(/[^0-9.-]+/g, "");
          const currencyAmount = parseFloat(currencyAmountStr);
          console.log(`Token ${token.ShortForm} currencyAmountStr:`, currencyAmountStr);
          console.log(`Token ${token.ShortForm} parsed currencyAmount:`, currencyAmount);
  
          if (!Number.isFinite(currencyAmount)) {
            const errorMsg = `Invalid currency amount for token ${token.ShortForm}: ${token.currencyAmount}`;
            console.error(errorMsg);
            return resolve(null);
          }
          const value = BigInt(Math.round(currencyAmount));
          console.log(`Token ${token.ShortForm} value (BigInt):`, value);
  
          // Convert CanisterId to Principal
          let CanisterId;
          try {
            CanisterId = Principal.fromText(token.CanisterId);
            console.log(`Token ${token.ShortForm} CanisterId (Principal):`, CanisterId.toText());
          } catch (error) {
            const errorMsg = `Invalid Canister ID for token ${token.ShortForm}: ${token.CanisterId}`;
            console.error(errorMsg);
            return resolve(null);
          }
  
          // Construct the pool data object
          const poolData = {
            weight: normalizedWeight,
            balance: balance,
            value: value,
            image: token.ImagePath || "",
            token_name: token.ShortForm || "Unnamed Token",
            ledger_canister_id: CanisterId,
          };
  
          resolve(poolData);
        });
      });
  
      // Wait for all token processing to complete
  
      return Promise.allSettled(pool_data)
        .then(async (results) => {
          let validPoolData = [];
          results.map((data, i) =>
            validPoolData.push({ weight: data.value.weight, balance: data.value.balance, value: data.value.value, image: data.value.image, token_name: data.value.token_name, ledger_canister_id: data.value.ledger_canister_id })
          )
          console.log("Pool data after processing tokens:", results.length);
  
  
          console.log("Valid pool data:", validPoolData);
  
          if (validPoolData?.length !== results.length) {
            const errorMsg = "Error processing tokens. Aborting pool creation.";
            console.log(validPoolData?.length, results.length)
            console.error(errorMsg);
            return Promise.reject(new Error(errorMsg));
          }
  
          if (FeeShare === undefined || FeeShare === null) {
            const errorMsg = "FeeShare is undefined or null";
            console.error(errorMsg);
            return Promise.reject(new Error(errorMsg));
          }
          const feeShareStr = FeeShare.toString().replace(/[^0-9.-]+/g, "");
          const swap_fee = parseFloat(feeShareStr);
          console.log("FeeShare string:", feeShareStr);
          console.log("Parsed swap_fee:", swap_fee);
  
          if (!Number.isFinite(swap_fee)) {
            const errorMsg = `Invalid swap fee: ${FeeShare}`;
            console.error(errorMsg);
            return Promise.reject(new Error(errorMsg));
          }
  
          // Combine pool_data and swap_fee into the expected structure
          const poolDetails = { pool_data: validPoolData, swap_fee };
          setSelectedTokenDetails(validPoolData); // Update state with the pool data
          console.log("Final poolDetails to be sent to backend:", poolDetails);
  
          // Check backendActor and create_pools method
          if (!backendActor || !backendActor.create_pools) {
            const errorMsg = "Backend actor is not available or create_pools method is missing";
            console.error(errorMsg);
            return Promise.reject(new Error(errorMsg));
          }
  
          // Call the backend to create the pool
          return backendActor.create_pools(poolDetails)
            .then((result) => {
              console.log("Backend response:", result);
  
              if (result && 'Ok' in result) {
                // Treat any presence of "Ok" as success, even if it's null
                console.log("Pool created successfully");
                setPoolCreated(true); // Update state on success
                return Promise.resolve(result);
              } else if (result && result.Err) {
                console.error("Error creating pool:", result.Err); // Log the error message from the backend
                return Promise.reject(new Error(result.Err));
              } else {
                const errorMsg = `Unexpected response from backend: ${JSON.stringify(result)}`;
                console.error(errorMsg); // Log any unexpected response
                return Promise.reject(new Error(errorMsg));
              }
            });
        })
        .catch((error) => {
          console.error("Error while creating pool:", error);
          return Promise.reject(error);
        });
    };
  
  
  
    const finalPoolCreationghanlder = () => {
    //   handleOpenModal();
      const backendCanisterID = process.env.CANISTER_ID_VALUESWAP_BACKEND;
      console.log("Backend Canister ID:", backendCanisterID);
  
      console.log("Starting handleCreatePoolClick");
  
      handleCreatePoolClick(backendCanisterID)
        .then(poolClickResult => {
          console.log("handleCreatePoolClick(approval) result:", poolClickResult);
  
          if (poolClickResult.success) {
            console.log("Starting createPoolHandler");
  
            // Call createPoolHandler and chain the Promise
            return createPoolHandler()
              .then(createPoolResult => {
                console.log("createPoolHandler result:", createPoolResult);
  
                if (createPoolResult || createPoolResult.Ok) {
                  setConfirmPool(true);
                  console.log("Pool creation confirmed");
                } else {
                  console.error("createPoolHandler failed:", createPoolResult.error);
                  return Promise.reject(new Error(createPoolResult.error));
                }
              });
          } else {
            console.error("handleCreatePoolClick indicated failure:", poolClickResult.error);
            if (poolClickResult.details) {
              poolClickResult.details.forEach(detail => {
                console.error(`Token ${detail.token.CanisterId} failed: ${detail.error}`);
              });
            }
            return Promise.reject(new Error(poolClickResult.error));
          }
        })
        .catch(error => {
          console.error("Pool creation process failed:", error);
          // Optionally, handle the error here (e.g., show a notification to the user)
        });
    };
  
  return (
    <div onClick={finalPoolCreationghanlder}>
         <GradientButton CustomCss={` w-full md:w-full`}>{buttonText}</GradientButton>
    </div>
  )
}

export default Approval