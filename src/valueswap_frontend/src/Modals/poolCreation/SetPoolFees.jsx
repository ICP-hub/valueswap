import React, { useEffect, useState } from 'react'
import { Bolt } from 'lucide-react';
import SearchTokenShowData from '../../components/searchTokenForPoolComponents/SearchTokenShowData';
import GradientButton from '../../buttons/GradientButton';
import { showAlert, hideAlert } from '../../reducer/Alert';
import { useDispatch } from 'react-redux';
import { SetFeeShare } from '../../reducer/PoolCreation';
import BorderGradientButton from '../../buttons/BorderGradientButton';
import { toast } from 'react-toastify';

const SetPoolFees = ({ handleNext, setFixedActiveSetp }) => {

    const dispatch = useDispatch();
    const [ButtonActive, SetButtonActive] = useState(false);
    const [selectedIndex, setSelectedIndex] = useState(null);
    const PercentShares = [0.1, 0.30, 0.50, 1.00];
    useEffect(() => {

        if (selectedIndex === null) {
            SetButtonActive(false);
        } else {
            SetButtonActive(true);
        }


        if (selectedIndex === null) {
            dispatch(SetFeeShare(
                {
                    FeeShare: 0
                }
            ))
        } else {
            dispatch(SetFeeShare(
                {
                    FeeShare: PercentShares[selectedIndex]
                }
            ))
        }
    }, [selectedIndex, selectedIndex])

    const HandleClick = (index) => {
        setSelectedIndex(selectedIndex === index ? null : index);
    }


    return (
        <div className=''>
                <div className='w-full'>
                <div className={`flex gap-6 pb-6 w-[70%] md:w-[60%] justify-between items-center m-auto  lg:hidden`} >
                    <div className={`py-2 px-4 rounded-full bg-[#F7931A]`}>2</div>
                    <p className="text-lg"></p>
                    <hr className="border-2 w-3/4 pr-6" />
                </div>
            </div>
            <div className='z-50 w-min md:w-max m-auto flex flex-col items-start gap-4 p-4 sm:p-6 border mx-auto rounded-xl backdrop-blur-[32px]'>
                <span className='font-gilroy font-semibold text-xl sm:text-2xl'>Set Initial Liquidity</span>



                <div className='text-start font-gilroy font-semibold text-base sm:text-xl leading-7 tracking-wider '>
                    Initial Swap Fee
                </div>

                <div className='font-normal leading-5 font-gilroy text-sm sm:text-base tracking-wide max-w-[600px]'>
                    The ideal swap fee of 0.30% works well for pools with popular tokens. For pools containing less common tokens, consider raising the fee.
                </div>


                <div className='grid grid-cols-12 text-center gap-6 my-6'>
                    {PercentShares.map((share, index) => {
                        return (
                            <div
                                key={index}
                                onClick={() => {
                                    HandleClick(index);
                                }}
                                className=' col-span-6 sm:col-span-3'
                            >
                                <BorderGradientButton  customCss={`w-8 ${selectedIndex === index ? 'px-[4.3rem] w-0 bg-[#C16800] ' : 'px-[4.3rem] w-0 bg-[#3E434B]'}`}>
                                    <div className='flex justify-center'>
                                     {share}
                                     <span>%</span>
                                    </div>
                                </BorderGradientButton>
                            </div>
                        );
                    })}
                    
                </div>

            </div>
                <div
                    className={`font-gilroy text-base font-medium flex justify-center`}
                    onClick={() => {

                        if (!ButtonActive) {
                            toast.warn('Please select a fee tier.')
                        } else {
                            setFixedActiveSetp(2)
                            handleNext()
                        }
                    }}
                >
                    <GradientButton CustomCss={`my-4 custom-400:w-1/2 w-full  ${ButtonActive ? ' opacity-100 cursor-pointer' : 'opacity-50 cursor-default'}`}>
                        Next
                    </GradientButton>
                </div>
        </div>
    )
}

export default SetPoolFees
