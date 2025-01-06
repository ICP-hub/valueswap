import React, { useEffect, useState } from 'react'
import { Copy, Check } from 'lucide-react';
import { showAlert, hideAlert } from '../reducer/Alert';
import { useDispatch } from 'react-redux';
import ArrowCircleRightOutlinedIcon from '@mui/icons-material/ArrowCircleRightOutlined';
import GradientButton from '../buttons/GradientButton';
import DarkModeToggle from "./DarkModeToggle"
import onClickOutside from 'react-onclickoutside';
import { useAuth } from '../components/utils/useAuthClient';
import { toast } from 'react-toastify';
import { IOSSwitch } from '../buttons/SwitchButton';

function Profile({ Principal, isAuthenticated, logout, principal }) {
    const [showProfile, setShowProfile] = useState(false)
    const [copied, setCopied] = useState(false);
    const [isDarkMode, setIsDarkMode] = useState(false);
    const dispatch = useDispatch()
    const { balance } = useAuth()



    //    console.log("balance", balance)
    const CopyprincipleId = () => {
        navigator.clipboard.writeText(principal)
            .then(() => {
                console.log('Text copied to clipboard:', principal);

            })
            .catch(err => {
                console.error('Unable to copy text to clipboard:', err);
            });

        toast.success('Copied to ClipBoard',{ position: "top-center"})
        setCopied(true);

        setTimeout(() => {
            setCopied(false);
        }, 2000)
    };



    Profile.handleClickOutside = () => {
        setShowProfile(false);
    };

    console.log("principal, ", principal)

    return (
        <div className='relative '>
            <div className='flex gap-x-4 justify-center'>
                <p className='font-medium self-center'>{Principal}</p>
                {/* <div>
                    <p className='bg-gradient-to-r from-[#F7931A] via-[#767DFF] to-[#00308E] bg-clip-text text-transparent'>2.2501 ETH</p>
                </div> */}
                <img src="/image/Ellipse.png" alt="" className='' onClick={() => setShowProfile((prev) => !prev)} />
            </div>
            {showProfile ? <div className='absolute font-gilroy bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-10 border  border-[#FFFFFF66] rounded-2xl p-8 mb-8 top-16 w-[27vw] lg:w-[20vw] right-[-2rem]  py-2 px-2 lg:px-4 flex flex-col gap-y-4'>
                <div className='flex gap-x-4 w-full'>
                    <img src="/image/Ellipse.png" alt="" className='' />
                    <div className='w-full flex'>
                        {
                            isAuthenticated && <div className='flex w-full flex-row items-center justify-between text-center text-white font-gilroy text-xl font-normal'>
                                <span>
                                    {Principal}
                                </span>
                                {
                                    copied ? (
                                        <span className='cursor-pointer'>
                                            <Check size={18} />
                                        </span>
                                    ) : (
                                        <span className='cursor-pointer'
                                            onClick={
                                                () => {
                                                    CopyprincipleId()
                                                }
                                            }>
                                            <Copy size={18} />
                                        </span>
                                    )


                                }
                            </div>
                        }
                        
                    </div>
                </div>
                <hr />
                <div className='flex justify-between items-center'>
                    <p>Testnet mode</p>
                     <IOSSwitch sx={{ m: 1 }}   />
                </div>
                
                <div className='w-full'>
                    <GradientButton CustomCss={`w-full md:w-full h-[37px] font-[500]`} onClick={() => logout()}>
                        Log out
                    </GradientButton>
                </div>
            </div> : ""}
        </div>
    )
}

const clickOutsideConfig = {
    handleClickOutside: () => Profile.handleClickOutside,
};

export default onClickOutside(Profile, clickOutsideConfig);

// ${isDarkMode ? 'bg-gray-900' : 'bg-gray-100'}