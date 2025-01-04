import React, { useState } from 'react'
import InfoOutlinedIcon from '@mui/icons-material/InfoOutlined';
import BorderGradientButton from '../buttons/BorderGradientButton';

function SwapSetting() {
    const [percentage, setPercentage] = useState({
        value: 0,
    });

    const [showTooltip, setShowTooltip] = useState(false);
    const [showInput, setShowInput] = useState(false);
    const [showTooltip2, setShowTooltip2] = useState(false)

    const handleMouseEnter = () => {
        setShowTooltip(true);
    };

    const handleMouseLeave = () => {
        setShowTooltip(false);
    };

    const handleMouseEnterInfo2 = () => {
        setShowTooltip2(true);
    }

    const handleMouseLeaveInfo2 = () => {
        setShowTooltip2(false);
    }

    const handleShowInput = () => {
        setShowInput((prev) => !prev);
        console.log("hii i got u")
    }

    const handlePercentageChange = (e) => {
        const inputValue = e.target.value;
        setPercentage({ ...percentage, value: inputValue })
        console.log(percentage);
    };


    return (
        <div className='w-full  h-fit z-50  ext-[#A3A3A3] bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-md bg-opacity-20 border  border-[#FFFFFF66] rounded-2xl relative'>
            <h1 className='font-gilroy text-3xl font-light text-center py-4'>Settings</h1>
            {showTooltip && (
                <div className='absolute right-1 top-6 p-4 bg-gray-700 w-[312px] z-50'>
                    <p className=''>Your Transaction will be roll backed if the price changes by more then choosen tolerance percentage</p>
                </div>
            )}
            <div className='h-[1px] w-full bg-custom-radial ' />
            <div className='flex flex-col items-start justify-start p-4 '>
                <h1 className='text-2xl font-gilroy font-medium pb-3'>Transaction Settings</h1>


                <div className='flex gap-1 md:flex-row flex-col  md:justify-between w-full items-center gap-y-5 pb-4'>
                    <div className='flex gap-x-1 justify-between md:justify-normal w-full'>
                        <div className='flex gap-x-1'>
                            <h3 >Slippage Tolerance</h3>
                            <p className='text-[#F7931A] inline-block md:hidden'>0.5%</p>
                        </div>

                        <div id='info' onMouseEnter={handleMouseEnter} onMouseLeave={handleMouseLeave} onClick={() => setShowTooltip(!showTooltip)}>
                            <InfoOutlinedIcon sx={{ color: '#FFFFFFBF' }} />
                        </div>
                        <p className='text-[#F7931A] hidden md:inline-block'>0.5%</p>
                    </div>
                    <div className='flex items-start w-full md:justify-end gap-x-4'>
                        {/* buttons */}
                        <button className='h-[40px] w-[87px] button-gradient-wrapper text-white font-[400] text-base font-gilroy rounded-lg py-4 px-[1.875rem] hover:opacity-50'><span className="button-gradient-content flex justify-center items-center p-1 ">
                            Auto
                        </span></button>
                        {/* <button className={` h-[39px] w-[92px] button-gradient-wrapper text-white font-[400] text-base font-gilroy rounded-lg py-4 px-[1.875rem] hover:opacity-50`} onClick={handleShowInput}>
                            <span className="button-border-gradient-content flex justify-center items-center">
                                Custom
                            </span>
                        </button> */}
                            <div  onClick={handleShowInput}>
                        <BorderGradientButton customCss={`bg-gray-700 h-[38px] md:h-[38px]`}>
                            Custom
                        </BorderGradientButton>
                            </div>
                    </div>
                </div>
                {
                    showInput && (
                        <div className='w-full pb-4 relative'>
                            <input className='bg-[#30303080] w-full py-2 px-2 rounded-lg outline-none' type="number" step="0.01" value={percentage?.value} onChange={handlePercentageChange} />
                            <span className="absolute right-4  transform  top-[15%] text-white">%</span>
                        </div>
                    )
                }
                <div className='flex flex-col md:flex-row gap-1 justify-between w-full items-center  pb-4'>
                    <div className='flex gap-x-1 pb-4 w-full justify-between md:justify-normal'  onMouseEnter={handleMouseEnterInfo2} onMouseLeave={handleMouseLeaveInfo2} onClick={() => setShowTooltip2(!showTooltip2)}>
                        <h3 >Transaction Validity</h3>
                        <div id='valid'>

                            <InfoOutlinedIcon sx={{ color: '#FFFFFFBF' }} />
                        </div>

                    </div>
                    <div className='flex gap-x-3 items-center justify-start md:justify-end w-full md:w-fit'>
                        {/* buttons */}

                        {/* <div className={` h-[39px] w-[59px] button-gradient-wrapper text-white font-[400] text-base font-gilroy rounded-lg py-4 px-[1.875rem] hover:opacity-50`}>
                           

                        </div> */}
                         <BorderGradientButton customCss={` h-[38px] md:h-[38px] bg-gray-700 `}>
                         <input type='number' maxLength="3" className=" bg-gray-700 flex w-8 justify-center items-center appearance-none	outline-none text-center" />

                         </BorderGradientButton>
                        <div >minutes</div>
                    </div>
                </div>

            </div>
            {showTooltip2 && (
                <div className='absolute right-1 bottom-2 sm:bottom-6 p-4 bg-gray-700 w-[312px] z-50'>
                    <p className=''>Your Transaction will be roll backed if it is pending for more then this time.</p>
                </div>
            )}
        </div>
    )
}

export default SwapSetting