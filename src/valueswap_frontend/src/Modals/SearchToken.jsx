import React, { useEffect, useState } from "react";
import { X } from "lucide-react";
import { SearchTokenData } from "../TextData";
import { useSelector } from "react-redux";
import { useAuth } from "../components/utils/useAuthClient";
import SearchIcon from "@mui/icons-material/Search";
import { fetchCoinGeckoData, searchCoinGeckoById } from "../components/utils/fetchCoinGeckoData";
import Skeleton from '@mui/material/Skeleton';
import Stack from '@mui/material/Stack';
import CloseIcon from '@mui/icons-material/Close';
import Box from '@mui/material/Box';


const SearchToken = ({ setSearchToken, setPayToken, setRecToken, id, setTokenData }) => {
  const { createTokenActor } = useAuth();
  const { Tokens } = useSelector((state) => state.pool);

  const [TokenOption, SetTokenOption] = useState(null);
  const [searchQuery, setSearchQuery] = useState("");
  const [allTokens, setAllTokens] = useState([]);
  const [filteredTokens, setFilteredTokens] = useState([]);
  const [canisterIdToken, setCanisterIdToken] = useState([]);
  const [metaData, setMetadata] = useState(0)
  const HandleClickToken = (index) => {
    SetTokenOption(TokenOption === index ? null : index);
  };

  // Fetch all tokens data
  useEffect(() => {
    const fetchListOfCoins = async () => {
      try {
        const fetchedListOfData = await fetchCoinGeckoData();
        setAllTokens(fetchedListOfData);
        setFilteredTokens(fetchedListOfData);
      } catch (error) {
        console.error("Error fetching data:", error);
      }
    };
    fetchListOfCoins();
  }, []);

  // Filter tokens based on search query
  useEffect(() => {
    if (searchQuery.trim() !== "") {
      const filtered = allTokens.filter((token) => {
        const tokenName = token.name || "";
        const tokenSymbol = token.symbol || "";
        return (
          tokenName.toLowerCase().includes(searchQuery.toLowerCase()) ||
          tokenSymbol.toLowerCase().includes(searchQuery.toLowerCase())
        );
      });
      setFilteredTokens(filtered);
    } else {
      setFilteredTokens(allTokens);
    }
  }, [searchQuery, allTokens]);

  
  const fetchMetadata = async (CanisterId) => {
    try {
      const ledgerActor = await createTokenActor(CanisterId);
      const result = await ledgerActor?.icrc1_metadata();
      console.log("Fetched metadata:", result);
  
      // Extract decimals and symbol from the metadata
      const decimalsEntry = result.find(([key]) => key === "icrc1:decimals");
      const symbolEntry = result.find(([key]) => key === "icrc1:symbol");
  
      const decimals = decimalsEntry ? Number(decimalsEntry[1]?.Nat) : null; // Convert BigInt to Number
      const symbol = symbolEntry ? symbolEntry[1]?.Text : null;
      console.log("meta", decimals, symbol)
      return {
        decimals,
        symbol,
      };
    } catch (error) {
      console.error("Error fetching metadata:", error);
      return null; // Return null in case of an error
    }
  };
  
  
  
  // Fetch token containing mainnet Canister ID
  const fetchCanisterIdHandler = async (tokenName) => {
    try {
      const fetchResult = await searchCoinGeckoById(tokenName);
      if (!fetchResult) {
        throw new Error("Failed to fetch Canister ID");
      }
      console.log("Fetched Canister ID:", fetchResult);
      setCanisterIdToken([fetchResult]);
      return fetchResult;
    } catch (error) {
      console.error("Error in fetchCanisterIdHandler:", error);
      return null; // Return null in case of an error
    }
  };
     
   


  return (
    <div className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-screen h-screen overflow-y-auto text-[#FFFFFF] flex items-center justify-center bg-black bg-opacity-50 backdrop-blur-sm z-50">
      <div className="h-3/4 w-[90%] sm:w-[480px] bg-[#3B3D41] rounded-2xl shadow-lg flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center px-4 pt-6 border-b border-gray-700">
          <h2 className="font-gilroy text-xl text-white">{SearchTokenData.Heading}</h2>
          <button
            onClick={() => setSearchToken(false)}
            className="text-gray-400 hover:text-white"
          >
            <CloseIcon />
          </button>
        </div>

        {/* Search Bar */}
        <div className="flex items-center px-4 py-4 relative">
          <div className="absolute pl-4">
            <SearchIcon fontSize="medium" sx={{ color: '#C0D9FF66' }} />
          </div>
          <input
            type="text"
            placeholder="Search token by name or symbol"
            className="w-full p-3 pl-10 text-white bg-transparent rounded-xl outline-none border-2 border-[#C0D9FF66] placeholder-gray-400"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />

        </div>

        {/* Token List */}
        <div className="px-4 py-2 overflow-scroll h-full">
          {filteredTokens.length > 0 ? (
            filteredTokens.map((token, index) => {

              // Extract data from token
              const TokenName = token.name || '';
              const TokenId = token.id || '';
              const ShortForm = token.symbol ? token.symbol : '';
              const ImagePath = token.image?.large || token.image || token.thumb || token.large || '';
              const marketPrice = token.current_price || token.market_data?.current_price?.usd || '-';

              // Find corresponding metadata if available(mainnet)
              // const CanisterId = canisterIdToken? token.contract_address : null;
              const CanisterId = ShortForm == "cketh" ? process.env.CANISTER_ID_CKETH : process.env.CANISTER_ID_CKBTC;
              //  setMetadata()

              // Find the amount based on CanisterId
              const findAmount = Tokens?.find((t) => t?.CanisterId === CanisterId);
              const TokenAmount = findAmount ? findAmount.Amount : 0;


              return (
                <div
                  key={index}
                  className={`flex items-center gap-3 p-3 rounded-lg cursor-pointer `}
                  onClick={async () => {
                    const fetchResult = await fetchCanisterIdHandler(token.id);
                    if (!fetchResult) {
                      console.error("Failed to fetch Canister ID for token:", token.id);
                      return;
                    }
                    const meta = await  fetchMetadata(CanisterId);
                    if (fetchResult && meta) {
                      const tokenData = {
                        id: token.id,
                        Name: TokenName,
                        ImagePath: ImagePath,
                        ShortForm: ShortForm,
                        metaData: meta || [],
//                         CanisterId: CanisterId,
                        // CanisterId: ShortForm == "cketh" ?  process.env.CANISTER_ID_CKETH : process.env.CANISTER_ID_CKBTC,


                        // CanisterId: CanisterId,
                        CanisterId: ShortForm == "cketh" ?  process.env.CANISTER_ID_CKETH : process.env.CANISTER_ID_CKBTC,

                        marketPrice: marketPrice,
                      };
                      if (id === 1) setPayToken(tokenData);
                      else if (id === 2) setRecToken(tokenData);
                      else if (id === 3) setTokenData(tokenData);

                      HandleClickToken(index);
                      setSearchToken(false);
                    }
                  }}
                >
                  <img
                    src={ImagePath}
                    alt={TokenName}
                    className="w-8 h-8 rounded-full"
                  />
                  <div>
                    <p className="text-white">{TokenName}</p>
                    <p className="text-gray-400 text-sm uppercase">{ShortForm}</p>
                  </div>
                </div>
              );
            })
          ) : (
            <div>
            {Array.from({ length: 5 }).map((_, index) => (
              <Box
                key={index}
                sx={{ display: "flex", alignItems: "center", mb: 1, gap: 2 }} // Add margin between elements
                spacing={1}
              >
                <Skeleton variant="circular" width={40} height={40} />
                <Box>
                  <Skeleton variant="text" sx={{ fontSize: "1rem" }} width={350} height={30}/>
                  <Skeleton variant="text" sx={{ fontSize: "1rem" }} width={70} />
                </Box>
              </Box>
            ))}
          </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default SearchToken;
