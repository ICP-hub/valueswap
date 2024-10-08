import React, { useEffect, useState } from 'react';
import { X } from 'lucide-react';
import { SearchTokenData, DummyDataTokens } from '../TextData';
import { useSelector } from 'react-redux';
import { useAuth } from '../components/utils/useAuthClient';
import SearchIcon from '@mui/icons-material/Search';
import { fetchCoinGeckoData, searchCoinGeckoById } from '../components/utils/fetchCoinGeckoData';
import Skeleton from 'react-loading-skeleton';

const SearchToken = ({ setSearchToken,searchToken, setPayToken, setRecToken, id, setTokenData }) => {
  const { createTokenActor } = useAuth();
  const { Tokens } = useSelector((state) => state.pool);

  const [TokenOption, SetTokenOption] = useState(null);
  const [searchQuery, setSearchQuery] = useState('');
  const [metadata, setMetadata] = useState([]);
  const [allTokens, setAllTokens] = useState([]);
  const [filteredTokens, setFilteredTokens] = useState([]);
  const [canisterIdToken, setCanisterIdToken] = useState([]);

  const HandleClickToken = (index) => {
    SetTokenOption(TokenOption === index ? null : index);
  };

  // Fetch all tokens data from CoinGecko API using fetchCoinGeckoData
  useEffect(() => {
    const fetchListOfCoins = async () => {
      try {
        const fetchedListOfData = await fetchCoinGeckoData();
        setAllTokens(fetchedListOfData);
        setFilteredTokens(fetchedListOfData);
      } catch (error) {
        console.error('Error fetching data:', error);
      }
    };
    fetchListOfCoins();
  }, []);

  // Filter tokens based on search query
  useEffect(() => {
    if (searchQuery.trim() !== '') {
      const filtered = allTokens.filter((token) => {
        const tokenName = token.name || '';
        const tokenSymbol = token.symbol || '';
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

  // Fetch metadata for tokens when canisterIdToken changes
  useEffect(() => {
    const fetchMetadata = async () => {
      try {
        const fetchedMetadata = await Promise.all(
          canisterIdToken.map(async (token) => {
            console.log('ledger', token.contract_address);
            const ledgerActor = await createTokenActor(token?.contract_address);
            console.log('ledgerActor', ledgerActor);
            const result = await ledgerActor?.icrc1_metadata();
            console.log('metadata result', result);
            return {
              CanisterId: token?.contract_address,
              id: token.id,
              image: token.image,
              Name: token.name,
              metadata: result,
            };
          })
        );
        setMetadata(fetchedMetadata);
      } catch (error) {
        console.error('Error fetching metadata:', error);
      }
    };
    if (canisterIdToken.length > 0) {
      fetchMetadata();
    }
  }, [canisterIdToken, createTokenActor]);

  // Fetch token containing mainnet Canister ID
  const fetchCanisterIdHandler = async (tokenName) => {
    try {
      const fetchResult = await searchCoinGeckoById(tokenName);
      if (!fetchResult) {
        throw new Error('cannot call fetchCanisterIdHandler');
      }
      console.log('fetchCanisterIdHandler', fetchResult);
      // Ensure canisterIdToken is an array
      setCanisterIdToken([fetchResult]);
      return fetchResult;
    } catch (error) {
      console.log(error);
      return null;
    }
  };

  // const finalToken = canisterIdToken.length > 0 ? canisterIdToken : filteredTokens;

  return (
    <div className='flex z-50 justify-center fixed inset-0 bg-opacity-50 backdrop-blur-sm'>
      <div className='h-fit md:w-[40%] lg:w-[35%] border rounded-xl flex flex-col gap-2 bg-[#05071D] my-auto mx-auto'>
        <div className='w-[90%] flex justify-center mx-4'>
          <span className='font-fahkwang font-medium mx-auto md:text-2xl text-xl py-4'>
            {SearchTokenData.Heading}
          </span>
          <span className='cursor-pointer self-center' onClick={() => setSearchToken(false)}>
            <X />
          </span>
        </div>

        <div className='border border-transparent font-bold custom-height-3 bg-gradient-to-r from-transparent via-[#00308E] to-transparent w-full mx-auto'></div>
        <div className='flex m-4 w-10/12 mx-auto font-cabin font-normal text-xl'>
          <input
            type='text'
            placeholder='Search token by Name or Symbol'
            className='w-full rounded-s-lg outline-none text-white bg-[#303030] placeholder-gray-400 p-2'
            value={searchQuery}
            onChange={(e) => {
              setSearchQuery(e.target.value);
            }}
          />
          <div className='bg-[#C16800] rounded-e-lg px-5 py-2 items-center flex gap-x-1'>
            <div className='hidden lg:block'>
              <SearchIcon />
            </div>
            <button>Search</button>
          </div>
        </div>

        <div className='flex flex-col items-center gap-4 mb-10 overflow-y-scroll h-72'>
          {filteredTokens?.length > 0 ? (
            filteredTokens.map((token, index) => {
              // Extract data from token
              const TokenName = token.name || '';
              const TokenId = token.id || '';
              const ShortForm = token.symbol ? token.symbol : '';
              const ImagePath = token.image?.large || token.image || token.thumb || token.large || '';
              const marketPrice = token.current_price || token.market_data?.current_price?.usd || '-';

              // Find corresponding metadata if available(mainnet)
              // const CanisterId = canisterIdToken? token.contract_address : null;
              const CanisterId = ShortForm == "cketh" ? process.env.CANISTER_ID_CKETH_LEDGER : process.env.CANISTER_ID_CKBTC_LEDGER;


              // Find the amount based on CanisterId
              const findAmount = Tokens?.find((t) => t?.CanisterId === CanisterId);
              const TokenAmount = findAmount ? findAmount.Amount : 0;

              return (
                <div
                  className={`flex gap-6 items-center w-10/12 p-2 bg-[#303030] hover:opacity-80 cursor-pointer opacity-100 rounded-xl ${
                    TokenOption === index
                      ? 'font-bold opacity-100 border bg-gradient-to-r from-[#000711] via-[#525E91] to-[#000711]'
                      : ''
                  }`}
                  key={index}
                  onClick={async () => {
                    const fetchResult = await fetchCanisterIdHandler(TokenId);
                    if (fetchResult) {
                      const CanisterId = await fetchResult?.contract_address;
                      const tokenData = {
                        id: TokenId,
                        Name: TokenName,
                        ImagePath: ImagePath,
                        ShortForm: ShortForm,
                        // CanisterId: CanisterId,
                        CanisterId: ShortForm == "cketh" ?  process.env.CANISTER_ID_CKETH_LEDGER : process.env.CANISTER_ID_CKBTC_LEDGER,
                        marketPrice: marketPrice,
                        currencyAmount: marketPrice * TokenAmount,
                      };
                      console.log("result hai", tokenData)
                      console.log("result id", id)
                      if (id === 1) setPayToken(tokenData);
                      else if (id === 2) setRecToken(tokenData);
                      else if (id === 3) setTokenData(tokenData);

                      HandleClickToken(index);
                      setSearchToken(false);
                    } else {
                      console.error('Failed to fetch token details');
                    }
                  }}
                >
                  <div className='rounded-lg bg-[#3D3F47] p-2'>
                    <img src={ImagePath} alt='' className='h-6 w-6 transform scale-150' />
                  </div>
                  <div className='font-normal text-xl font-cabin text-start'>
                    {TokenName} ({ShortForm})
                  </div>
                </div>
              );
            })
          ) : (
            <div>
              <Skeleton count={5} />
            </div>
          )}
        </div>
        <div className='border border-transparent font-bold custom-height-3 bg-gradient-to-r from-transparent via-[#00308E] to-transparent w-full mx-auto'></div>
      </div>
    </div>
  );
};

export default SearchToken;

