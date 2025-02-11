import React, { useEffect, useState } from 'react';
import SearchToken from '../../Modals/SearchToken';
import BlueGradientButton from '../../buttons/BlueGradientButton';
import { ChevronDown, ChevronUp, Trash, LockKeyhole, LockKeyholeOpen } from 'lucide-react';
import { useDispatch, useSelector } from 'react-redux';
import { SetToken, RemoveCoin, ToggleLocked, setWeightedPercent } from '../../reducer/PoolCreation';
import { showAlert, hideAlert } from '../../reducer/Alert';

const SearchTokenShowData = ({ token, index, HandleSelectCheck }) => {
    const dispatch = useDispatch();
    const [TokenData, setTokenData] = useState({});
    const [searchToken, setSearchToken] = useState(false);
    const { CoinCount } = useSelector(state => state.pool);

console.log("searchToken", searchToken)

    const HandleData = (index, TokenData) => {
        if (TokenData.Name) {
            dispatch(SetToken({
                index: index,
                TokenData: TokenData
            }));
        }
    };
    const handleChangePercent = (e) => {
        dispatch(setWeightedPercent({
            index: index,
            percent: e.target.value
        }))
    };

    return (
        <div id='selectToken' className='flex justify-between gap-8 custom-400:gap-8 custom-450:gap-16 sm:gap-32 items-center mt-4 z-10' key={token.id}>
            <div className='flex justify-between items-center gap-1 sm:gap-2'>
                <span>{token.ShortForm}</span>
                <span className='bg-[#3E434B] py-1 rounded-lg px-1 md:px-3'>
                    <input
                        type="number"
                        className='bg-transparent w-10 text-base hide-arrows'
                        value={token.weights}
                        onChange={handleChangePercent}
                        disabled={token.weightsLocked}
                    />
                    <span className='md:text-lg text-xs'>%</span>
                </span>
                <span>
                    {
                        token.weightsLocked ? (
                            <span className='cursor-pointer'
                                onClick={() => {
                                    dispatch(ToggleLocked({
                                        index: index,
                                        toggle: false,
                                        percent: token.weights,
                                    }))
                                }}>
                                <LockKeyhole size={18} color="#C16800"/>
                            </span>
                        ) : (
                            <span className='cursor-pointer'
                                onClick={() => {
                                    dispatch(ToggleLocked({
                                        index: index,
                                        toggle: true,
                                        percent: token.weights,
                                    }))
                                }}>
                                <LockKeyholeOpen size={18} />
                            </span>
                        )
                    }
                </span>
                <span onClick={() => {
                    if (CoinCount > 2) {
                        dispatch(RemoveCoin({
                            index: index
                        }));
                    } else {
                        dispatch(showAlert({
                            type: 'danger',
                            text: 'Pool must have more than 1 coin'
                        }));
                        setTimeout(() => {
                            dispatch(hideAlert());
                        }, 3000);
                    }
                }} className='cursor-pointer'>
                    <Trash size={18} color="#eb3023"/>
                </span>

            </div>

            <div>
                {token.Selected ? (
                    <div className='flex flex-col gap-1'>
                        <div className='flex items-center place-self-end gap-1 custom-400:gap-2'>
                            <BlueGradientButton customCss={'disabled px-2 py-2  normal-cursor'}>
                                <img src={token.ImagePath} alt="" className='h-3 w-3 md:h-4 md:w-4 transform scale-150' />
                            </BlueGradientButton>

                            <div className='flex items-center gap-1'
                                onClick={() => {
                                    setSearchToken(!searchToken);
                                }}>
                                <div className='font-gilroy font-normal text-xl md:text-2xl cursor-pointer'>
                                    {token.ShortForm}
                                </div>
                                {!searchToken ? (
                                    <span className='cursor-pointer' ><ChevronDown size={18} /></span>
                                ) : (
                                    <span className='cursor-pointer' onClick={() => {
                                        setSearchToken(!searchToken);
                                    }}><ChevronUp size={18} /></span>
                                )}
                            </div>
                            <div className=''>
                                {searchToken && <SearchToken setSearchToken={setSearchToken} searchToken={searchToken} setTokenData={setTokenData} set id={3} />}
                            </div>
                            <div className='hidden'>
                                {HandleData(index, TokenData)}
                                {HandleSelectCheck()}
                            </div>
                        </div>
                    </div>
                ) : (
                    <div>
                        <div onClick={() => {
                        setSearchToken(true);
                    }}>

                        <BlueGradientButton customCss={'py-2 px-2 lg:px-4 lg:py-3 font-gilroy font-light'}  >
                            <div className='flex items-center gap-1 text-xs sm:text-sm'
                                >
                                Select a Token
                                <span className='cursor-pointer' ><ChevronDown size={18} /></span>
                            </div>
                        </BlueGradientButton>
                        </div>
                        {searchToken && <SearchToken setSearchToken={setSearchToken} setTokenData={setTokenData} set id={3} />}
                        {/* {console.log("index of the selected", index)} */}
                        {HandleData(index, TokenData)}
                        {HandleSelectCheck()}
                    </div>
                )}
            </div>
        </div>
    );
};

export default SearchTokenShowData;