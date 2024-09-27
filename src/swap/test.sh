dfx canister call swap store_pool_data '(
    principal "2sc6p-3wvn7-dd4xb-gfi24-3itu6-2ievx-gji66-y7zwu-jpkk5-cqb72-2ae", 
    record { 
        pool_data = vec { 
            record { 
                weight = 0.5; 
                balance = 1000; 
                value = 1000; 
                token_name = "eth"; 
                ledger_canister_id = principal "aaaaa-aa"; 
                image = "image_url" 
            }; 
            record { 
                weight = 0.5; 
                balance = 1000; 
                value = 1000; 
                token_name = "dai"; 
                ledger_canister_id = principal "aaaaa-aa"; 
                image = "image_url" 
            } 
        }; 
        swap_fee = 0.01 
    }
)'