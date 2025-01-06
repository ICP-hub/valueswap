import React, { useEffect, useState } from 'react'
import { Bolt } from 'lucide-react';
import BorderGradientButton from '../../buttons/BorderGradientButton'
import SearchTokenShowData from '../../components/searchTokenForPoolComponents/SearchTokenShowData';
import GradientButton from '../../buttons/GradientButton';
import { showAlert, hideAlert } from '../../reducer/Alert';
import { useDispatch, useSelector } from 'react-redux';
import { AddCoin } from '../../reducer/PoolCreation';
import { toast } from 'react-toastify';
const SelectTokensForPools = ({ handleNext, setFixedActiveSetp }) => {
    
    const dispatch = useDispatch();
    const { Tokens, CoinCount } = useSelector((state) => state.pool)
    // console.log(Tokens)
    const [ButtonActive, SetButtonActive] = useState(false);

    useEffect(() => {
        console.log("Current Token Count:->", CoinCount)
    }, [CoinCount])

    const HandleSelectCheck = () => {
        const allTokensSelected = Tokens.every((token) => token.Selected);
        // console.log("Selected or not->", allTokensSelected)
        SetButtonActive(allTokensSelected);
    }


    return (
        <div className='flex flex-col justify-center px-2 h-full'>
            <div className='w-full'>
                <div className={`flex gap-6 pb-6 w-[70%] md:w-[60%] justify-between items-center m-auto  lg:hidden`} >
                    <div className={`py-2 px-4 rounded-full bg-[#F7931A]`}>1</div>
                    <p className="text-lg"></p>
                    <hr className="border-2 w-3/4 pr-6" />
                </div>
            </div>
            <div className='inset-0 md:min-w-[500px] w-max bg-opacity-10 m-auto justify-center items-center flex flex-col gap-4 p-3 sm:p-6 border mx-auto rounded-xl backdrop-blur-[32px]'> 

                {/* <div className='w-[90%] place-self-end  flex justify-between px-6'>
                    <span className='font-gilroy font-light md:text-3xl '>Select Tokens</span>
                    <Bolt size={30} className='cursor-pointer' onClick={() => { console.log("settings open") }} />
                </div>
                 */}

                <div className='w-full flex justify-between items-center gap-4'>
                    <p className='font-gilroy text-2xl font-light'>Choose upto 8 tokens:</p>
                    <div className={`place-self-end ${CoinCount < 8 ? 'block' : 'hidden'}`}
                    onClick={() => {
                        if (CoinCount < 8) {
                            dispatch(AddCoin())
                        } else {
                            
                            toast.warn('No More than 8 coins Allowed in the pool')
                        }
                    }}
                >
                    <BorderGradientButton customCss={`bg-[#182030] text-xs md:text-light lg:text-base h-[45px] w-[115px] md:w-[140px] `}>
                        Add Token
                    </BorderGradientButton>
                </div>
                </div>


                <div className='flex flex-col items-center space-y-2'>
                    {Tokens.map((token, index) => {
                        return (
                            <div key={index} className='flex items-center align-middle space-x-2'>
                                <SearchTokenShowData token={token} index={index} HandleSelectCheck={HandleSelectCheck} />
                            </div>
                        );
                    })}
                </div>
            </div>
                <div
                    className={`font-gilroy text-base font-medium mx-auto`}
                    onClick={() => {

                        if (!ButtonActive) {
                            
                            toast.warn('Please select all the coins')
                        } else {
                            setFixedActiveSetp(1)
                            handleNext()
                        }
                    }}
                >
            <GradientButton CustomCss={`my-4 md:min-w-[500px] -z-50 ${ButtonActive ? ' opacity-100 cursor-pointer' : 'opacity-50 cursor-not-allowed'}`} >
                        Next
            </GradientButton>
            </div>
        </div>
    )
}

export default SelectTokensForPools
