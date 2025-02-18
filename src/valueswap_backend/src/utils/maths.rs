use candid::Nat;
use ic_cdk::query;
use num_traits::pow;
use num_traits::ToPrimitive;


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
// wI = tokenWeightIn               \      \       ( bI +  aI )        /              /
// wO = tokenWeightOut                                                                       //
// sF = swapFee                                                                              //
**********************************************************************************************/

#[query]
pub fn out_given_in(b_i: Nat, w_i: Nat, b_o: Nat, w_o: Nat, amount_in: Nat) -> Nat {
    // Define scaling factors
    let base_scaling = Nat::from(10u64.pow(18));  // 10^18 scaling for base
    let exp_scaling = Nat::from(100u64);          // scaling factor for exponent

    // Step 1: Calculate denominator
    let denominator = b_i.clone() + amount_in.clone();
    ic_cdk::println!("Step 1 - Denominator: {:?}", denominator);

    // Step 2: Calculate base with scaling
    let base = (b_i.clone() * base_scaling.clone()) / denominator.clone();
    ic_cdk::println!("Step 2 - Base: {:?}", base);

    // Step 3: Calculate exponent with scaling
    let exponent = (w_i.clone() * exp_scaling.clone()) / w_o.clone();
    ic_cdk::println!("Step 3 - Exponent: {:?}", exponent);

    // Step 4: Calculate power iteratively
    let mut power = base_scaling.clone(); // Start with scaled 1.0
    let mut temp_exp = exponent.clone();
    ic_cdk::println!("Step 4 - Initial Power: {:?}", power);

    while temp_exp > Nat::from(0u128) {
        power = (power * base.clone()) / base_scaling.clone();
        ic_cdk::println!("Step 4 - Power after iteration: {:?}", power);
        if temp_exp < exp_scaling {
            break; // Avoid subtracting below zero
        }
        temp_exp -= exp_scaling.clone();  // Decrement exponent by the scaling factor
    }

    // Step 5: Calculate complement of power
    let power_complement = base_scaling.clone() - power.clone();
    ic_cdk::println!("Step 5 - Power Complement: {:?}", power_complement);

    // Step 6: Calculate final output
    let output = (b_o.clone() * power_complement.clone()) / base_scaling.clone();
    ic_cdk::println!("Step 6 - Output: {:?}", output);

    output
}

// // Helper function to compute power (scaled base^scaled exponent)
// fn power_of(base: Nat, exponent: Nat, scaling_factor: Nat) -> Nat {
//     // Use logarithmic approximation or iterative multiplication for precision
//     let mut result = scaling_factor.clone(); // Start with 1 in scaled terms
//     let mut base_scaled = base;

//     // Convert exponent to an integer approximation
//     let exp_int = exponent.0.to_u128().unwrap_or(1);

//     for _ in 0..exp_int {
//         result = (result * base_scaled.clone()) / scaling_factor.clone();
//     }

//     result
// }

// use ic_cdk::export::candid::Nat;

// #[query]
// pub fn out_given_in(b_i: Nat, w_i: Nat, b_o: Nat, w_o: Nat, amount_in: Nat, fee: Nat) -> Nat {
//     // Scaling factor for precision (1e18 for fixed-point arithmetic)
//     let scaling_factor = Nat::from(10u128.pow(18));
//     let one = scaling_factor.clone();

//     // Apply fee adjustment: amount_in * (1 - fee/100)
//     let fee_adjusted_amount_in = amount_in.clone() * (one.clone() - fee.clone() * scaling_factor.clone() / Nat::from(100u128)) / scaling_factor.clone();

//     // Calculate the denominator: b_i + fee_adjusted_amount_in
//     let denominator = b_i.clone() + fee_adjusted_amount_in.clone();

//     // Calculate the base: b_i / denominator
//     let base = (b_i.clone() * scaling_factor.clone()) / denominator;

//     // Calculate the exponent: w_i / w_o
//     let exponent = (w_i.clone() * scaling_factor.clone()) / w_o.clone();

//     // Calculate the power: base^exponent
//     let power = pow1(base, exponent.clone(), scaling_factor.clone());

//     // Final calculation: b_o * (1 - power)
//     let result = b_o.clone() * (scaling_factor.clone() - power) / scaling_factor;

//     result
// }

// // Function to calculate base^exponent with rounding
// fn pow1(base: Nat, exponent: Nat, scaling_factor: Nat) -> Nat {
//     let mut result = scaling_factor.clone();
//     let mut base_scaled = base.clone();
//     let mut exp = exponent.0.to_u128().unwrap_or(0);

//     while exp > 0 {
//         if exp % 2 == 1 {
//             result = (result * base_scaled.clone()) / scaling_factor.clone();
//         }
//         base_scaled = (base_scaled.clone() * base_scaled.clone()) / scaling_factor.clone();
//         exp /= 2;
//     }

//     result
// }





#[query]
pub fn out_given_in_past(b_i: f64, w_i: f64, b_o: f64, w_o: f64, amount_in: f64, fee: f64) -> f64 {
    b_o * (
        1.0 - (b_i / (b_i + amount_in * (1.0 - fee/100.0))).powf(w_i / w_o) )
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

// fn power_of(base: Nat, exponent: Nat) -> Nat {
//     let mut result = Nat::from(1u128); 
//     let mut exp = exponent.clone(); 
    
//     while exp > Nat::from(0u128) {
//         result *= base.clone();
//         exp -= Nat::from(1u128);
//     }
    
//     result
// }