# dfx canister call valueswap_backend out_given_in_past '(5.0 : float64, 0.5 : float64, 141.0 : float64, 0.5 : float64, 1.0 : float64, 0.3 : float64)'

dfx canister call valueswap_backend out_given_in '(
    500000000 : nat,
    30 : nat,
    141000000000000000000 : nat,
    70  : nat,
    100000000 : nat
)'

dfx canister call valueswap_backend pre_compute_swap '(
    record {
        token1_name = "ckbtc";
        token_amount = 400000000 : nat;
        token2_name = "cketh";
        ledger_canister_id1 = principal "br5f7-7uaaa-aaaaa-qaaca-cai";
        ledger_canister_id2 = principal "bw4dl-smaaa-aaaaa-qaacq-cai";
    }
)'






