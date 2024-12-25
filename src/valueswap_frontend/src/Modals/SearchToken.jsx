import React, { useEffect, useMemo, useState } from 'react'
import { X } from 'lucide-react';
import { SearchTokenData, DummyDataTokens } from '../TextData';
import { useSelector } from 'react-redux';
// import GradientButton from '../buttons/GradientButton';
// import { SiBitcoinsv } from "react-icons/si";
// import { CiSearch } from "react-icons/ci";
import { useAuth } from '../components/utils/useAuthClient';
import SearchIcon from '@mui/icons-material/Search';
import { Principal } from '@dfinity/principal';
import { fetchCoinGeckoData, searchCoinGeckoById } from '../components/utils/fetchCoinGeckoData';
const SearchToken = ({ setSearchToken, setPayToken, setRecToken, id, setTokenData }) => {

    const { createTokenActor, principal } = useAuth();
    const { Tokens } = useSelector(state => state.pool)
    const [TokenOption, SetTokenOption] = useState(null);
    // const ImagePath = DummyDataTokens.Tokens[0].Image
    const [searchQuery, setSearchQuery] = useState('');
    const [metadata, setMetadata] = useState([]);
    const [filteredTokens, setFilteredTokens] = useState()
    // const [Amount, setAmount] = useState()
    const HandleClickToken = (index) => {
        SetTokenOption(TokenOption === index ? null : index);
    }

    useEffect(() => {
        if (searchQuery.trim() !== "") {
            setFilteredTokens(
                metadata.filter((tokenMeta) => {
                    const TokenName = tokenMeta.metadata[1][1].Text;
                    return TokenName.toLowerCase().includes(searchQuery.toLowerCase());
                })
            );
        } else {
            setFilteredTokens(filteredTokens);
        }
    }, [searchQuery, metadata]);

    useMemo(() => {
        const fetchListOfCoin = async () => {
            const fetchedListOfData = await fetchCoinGeckoData()
            setFilteredTokens(fetchedListOfData)
        }
        fetchListOfCoin()
    }, [])


    useEffect(() => {
        const fetchMetadata = async () => {
            const fetchedMetadata = await Promise.all(
                DummyDataTokens.Tokens.map(async (token) => {
                    const ledgerActor = await createTokenActor(token?.CanisterId);
                    const result = await ledgerActor?.icrc1_metadata();
                    
                  
                    // setAmount(findAmount?.Amount)
                    return {
                        CanisterId: token?.CanisterId,
                        id: token.id,
                        image: token.image,
                        Name: token.name,
                        metadata: result
                    };
                })
            );
            setMetadata(fetchedMetadata);
            // setFilteredTokens(fetchedMetadata);
        };

        fetchMetadata();

    }, [DummyDataTokens, Tokens]);


    useEffect(() => {

    })

    return (
        <div className='flex z-50 justify-center fixed inset-0  bg-opacity-50 backdrop-blur-sm'>
            <div className=' h-fit md:w-[40%] lg:w-[35%]  border rounded-xl flex flex-col gap-2 bg-[#05071D] my-auto mx-auto '>
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
                        placeholder='Search token by Name'
                        className='w-full   rounded-s-lg outline-none text-white bg-[#303030] placeholder-gray-400  p-2'
                        value={searchQuery}
                        onChange={(e) => {
                            setSearchQuery(e.target.value);
                        }}
                    />
                    <div className='bg-[#C16800] rounded-e-lg px-5 py-2 items-center flex gap-x-1'>
                        <div className='hidden lg:block'>
                            <SearchIcon />
                        </div>
                        <button >Search</button>

                    </div>
                </div>

                <div className='flex flex-col items-center gap-4 mb-10 overflow-y-scroll h-72'>
                    {filteredTokens?.length > 0 ?
                        filteredTokens.slice(0, 10).map((token, index) => {


                            const tokenMetadata = metadata?.find(meta => meta?.Name === token?.name);
                            // const TokenName = tokenMetadata?.metadata[1]?.[1]?.Text;
                            const TokenId = token.id;
                            const TokenName = token.name ? token.name : tokenMetadata?.metadata[1]?.[1]?.Text;
                            const CanisterId = tokenMetadata?.CanisterId;
                            const ShortForm = tokenMetadata?.metadata[2]?.[1]?.Text;
                            const ImagePath = token.image;
                            const findAmount = Tokens?.find(tokens => tokens?.CanisterId === CanisterId);
                            const TokenAmount = findAmount? findAmount.Amount : 0;
                            const marketPrice = token.current_price;
                            // const 

                            
                            return (
                                <div className={`flex gap-6 items-center w-10/12  p-2 bg-[#303030] hover:opacity-80 cursor-pointer  opacity-100 rounded-xl
                            ${TokenOption === index ? ' font-bold opacity-100 border bg-gradient-to-r from-[#000711] via-[#525E91] to-[#000711]' : ''}`} key={index}
                                    onClick={() => {
                                        if (id === 1) setPayToken({
                                            id: TokenId,
                                            Name: TokenName,
                                            ImagePath: ImagePath,
                                            ShortForm: ShortForm,
                                            CanisterId: CanisterId,
                                            marketPrice: marketPrice,
                                            currencyAmount: marketPrice * TokenAmount

                                        })
                                        if (id === 2) setRecToken({
                                            id: TokenId,
                                            Name: TokenName,
                                            ImagePath: ImagePath,
                                            ShortForm: ShortForm,
                                            CanisterId: CanisterId,
                                            marketPrice: marketPrice,
                                            currencyAmount: marketPrice * TokenAmount

                                        })
                                        if (id === 3) {
                                            setTokenData({
                                                id: TokenId,
                                                Name: TokenName,
                                                ImagePath: ImagePath,
                                                ShortForm: ShortForm,
                                                CanisterId: CanisterId,
                                                marketPrice: marketPrice,
                                                currencyAmount: marketPrice * TokenAmount

                                            })
                                        }
                                        HandleClickToken(index);
                                        setSearchToken(false)
                                    }}>
                                    <div className='rounded-lg bg-[#3D3F47] p-2'>
                                        <img src={ImagePath} alt="" className='h-6 w-6 transform scale-150' />
                                    </div>
                                    <div className='font-normal text-xl font-cabin text-start'>
                                        {TokenName}
                                    </div>
                                </div>
                            );
                        }) : <div> Loading.... </div>}
                </div>
                <div className='border border-transparent font-bold custom-height-3 bg-gradient-to-r from-transparent via-[#00308E] to-transparent w-full mx-auto'></div>

            </div>

        </div >
    )
}

export default SearchToken