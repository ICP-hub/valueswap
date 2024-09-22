


chmod 777 ./deploy_ckbtc.sh
./deploy_ckbtc.sh

dfx canister create xrc_demo
dfx deploy --with-cycles 10000000000

# If xrc demo canister fails to deploy 

# run: dfx info networks-json-path.
# navigate the directory 
# create a networks.json file if it dosen't exist and add the following code: 
# {
#     "local": {
#         "replica": {
#             "subnet_type": "system"
#         }
#     }
# }