use candid::Nat;

/**********************************************************************************************
// constantProduct                                                                           //
// This function calculates the constant product invariant for a given set of token balances //
// and their respective weights.                                                             //
//                                                                                           //
// balances = [b1, b2, b3, ..., b8]  (token balances)                                        //
// weights  = [w1, w2, w3, ..., w8]  (token weights)                                         //
//                                                                                           //
// constant_product V = (b1^w1) * (b2^w2) * (b3^w3) * ... * (b8^w8)                          //
**********************************************************************************************/

use std::f64::EPSILON;

pub fn constant_product(balances: &[f64], weights: &[f64]) -> f64 {
    let total_weight: f64 = weights.iter().sum();
    if (total_weight - 100.0).abs() > EPSILON {
        panic!("Sum of weights must be 100");
    }

    let normalized_weights: Vec<f64> = weights.iter().map(|&w| w / total_weight).collect();

    balances.iter()
        .zip(normalized_weights.iter())
        .map(|(&b, &w)| b.powf(w))
        .product()
}

/**********************************************************************************************
// calcSpotPrice                                                                             //
// sP = spotPrice                                                                            //
// bI = tokenBalanceIn                ( bI / wI )         1                                  //
// bO = tokenBalanceOut         sP =  -----------  *  ----------                             //
// wI = tokenWeightIn                 ( bO / wO )     ( 1 - sF )                             //
// wO = tokenWeightOut                                                                       //
// sF = swapFee                                                                              //
**********************************************************************************************/

// pub fn spot_price(b_i: f64, w_i: f64, b_o: f64, w_o: f64, fee: f64) -> f64 {
//     (b_i  / (w_i)) / (b_o / w_o) * (1.0 / (1.0 - fee))
// }

/**********************************************************************************************
// calcOutGivenIn                                                                            //
// aO = tokenAmountOut                                                                       //
// bO = tokenBalanceOut                                                                      //
// bI = tokenBalanceIn              /      /            bI             \    (wI / wO) \      //
// aI = tokenAmountIn    aO = bO * |  1 - | --------------------------  | ^            |     //
// wI = tokenWeightIn               \      \ ( bI + ( aI * ( 1 - sF )) /              /      //
// wO = tokenWeightOut                                                                       //
// sF = swapFee                                                                              //
**********************************************************************************************/

pub fn out_given_in(b_i: Nat, w_i: Nat, b_o: Nat, w_o: Nat, amount_in: Nat, fee: Nat) -> Nat {
    let temp = b_i.clone() / (b_i + amount_in * (Nat::from(1u128) - fee/Nat::from(100u128)));
    let exp = w_i/w_o;
    b_o * (Nat::from(1u128) - power_of(temp, exp))
}

/**********************************************************************************************
// calcInGivenOut                                                                            //
// aI = tokenAmountIn                                                                        //
// bO = tokenBalanceOut               /  /     bO      \    (wO / wI)      \                 //
// bI = tokenBalanceIn          bI * |  | ------------  | ^            - 1  |                //
// aO = tokenAmountOut    aI =        \  \ ( bO - aO ) /                   /                 //
// wI = tokenWeightIn           --------------------------------------------                 //
// wO = tokenWeightOut                          ( 1 - sF )                                   //
// sF = swapFee                                                                              //
**********************************************************************************************/

// pub fn in_given_out(b_i: f64, w_i: f64, b_o: f64, w_o: f64, amount_out: f64, fee: f64) -> f64 {
//     b_i * (((b_o / (b_o - amount_out)).powf(w_o / w_i)) - 1.0) / (1.0 - fee)
// }

/**********************************************************************************************
// calcAllAssetWithdraw                                                                      //
// This function calculates the balance of a single asset after withdrawing a certain amount //
// of total supply.                                                                          //
//                                                                                           //
// supply  = totalSupply                                                                     //
// redeemed = amountRedeemed                                                                 //
// bK = initialBalance                                                                       //
//                                                                                           //
// new_balance = (1.0 - (redeemed / supply)) * bK                                            //
**********************************************************************************************/

// pub fn all_asset_withdraw(supply: f64, redeemed: f64, b_k: f64) -> f64 {
//     (1.0 - redeemed / supply) * b_k
// }

/**********************************************************************************************
// calcSingleAssetWithdraw                                                                   //
// This function calculates the amount of a single asset that can be withdrawn given a       //
// specific amount of total supply and weight.                                               //
//                                                                                           //
// supply  = totalSupply                                                                     //
// redeemed = amountRedeemed                                                                 //
// bT = initialBalance                                                                       //
// wT = weight                                                                               //
//                                                                                           //
// withdraw_amount = bT * (1.0 - (1.0 - redeemed / supply).powf(1.0 / wT))                    //
**********************************************************************************************/

// pub fn single_asset_withdraw(supply: f64, redeemed: f64, b_t: f64, w_t: f64) -> f64 {
//     b_t * (1.0 - (1.0 - redeemed / supply).powf(1.0 / w_t))
// }


// pub fn convert_nat_to_u64(nat: Nat) -> Result<f64, &'static str> {
//     // Convert the Nat to a string
//     let nat_str = nat.to_string();

//     // Try to parse the string as a u64
//     match nat_str.parse::<f64>() {
//         Ok(value) => Ok(value),
//         Err(_) => Err("Nat value is too large for u64"),
//     }
// }

fn power_of(base: Nat, exponent: Nat) -> Nat {
    let mut result = Nat::from(1u128); 
    let mut exp = exponent.clone(); 
    
    while exp > Nat::from(0u128) {
        result *= base.clone();
        exp -= Nat::from(1u128);
    }
    
    result
}