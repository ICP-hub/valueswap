import React, { useState, useEffect, useMemo } from 'react';
import { Link as RouterLink, useLocation, useNavigate } from 'react-router-dom';
import GradientButton from '../buttons/GradientButton';
import { Bars3BottomRightIcon, XMarkIcon } from '@heroicons/react/24/solid';
import Profile from './Profile';
import { useSelector } from 'react-redux';
import { Principal } from '@dfinity/principal';
import { useAuth } from '../components/utils/useAuthClient';
import BorderGradientButton from '../buttons/BorderGradientButton';
const options = [
    // { value: 'ethereum', label: 'Ethereum', img: '/src/assets/images/Network/Ethereum.png' },
    // { value: 'bitcoin', label: 'Bitcoin', img: 'images/Bitcoin.png' },
    // Add more options here
];

const MobileNavbar = ({ NavbarData, setClickConnectWallet }) => {
    const [activeLink, setActiveLink] = useState();
    const [open, setOpen] = useState(false);
    const [Principal, setPrincipal] = useState()
    const [selectedOption, setSelectedOption] = useState(options[0]);
    const [isSticky, setIsSticky] = useState(true);

    let location = useLocation()

    // const walletId = localStorage.getItem('dfinityWallet') || '';
    // console.log(walletId)
    // if(walletId){
    //     console.log("we are connected!")
    //     artemisWallet()
    // }
    const { isAuthenticated, login, logout, principal, reloadLogin } = useAuth();

    useMemo(() => { 
        const getDisplayFunction = () => {
          console.log('principal:', principal);
          console.log('Type of principal:', typeof principal);
        //   console.log('Is principal an instance of Principal:', principal instanceof Principal);
      
          // Convert the Principal object to a string
          const principalString = principal.toText();
      
          // Format the principal string for display
          const SlicedPrincipal = principalString.slice(0, 5);
          const FinalId = SlicedPrincipal.padEnd(10, '.') + principalString.slice(-3);
          setPrincipal(FinalId);
          console.log("Principal of user is:", FinalId);
        }
      
        if (principal) {
          getDisplayFunction();
        }
      }, [principal]);
      
    const navigate = useNavigate();

    useEffect(() => {
        // This effect will run when the location changes
        setActiveLink(location.pathname);
    }, [location]);


    useEffect(() => {
        const handleScroll = () => {
            if (window.scrollY < 100) {
                setIsSticky(true);
            } else {
                setIsSticky(false);
            }
        };

        window.addEventListener('scroll', handleScroll);

        return () => {
            window.removeEventListener('scroll', handleScroll);
        };
    }, []);


    // console.log("isAuthenticated", isAuthenticated, principal)
    console.log("open", window.scrollY, isSticky )


    return (
        <div className={`  ${isSticky ? ' sticky top-0 transition-all delay-75 duration-300 ' : ' sticky top-[-150px] delay-300 duration-700'} z-50 px-4 md:px-8 `}>

            <div className="flex justify-center  font-gilroy   ">

                {/* mobile Navbar hamburgur */}
                <ul className={`md:hidden md:items-center  md:pb-0 pb-12 absolute md:static rounded-lg left-0 w-full md:w-auto md:pl-0  transition-all duration-700 ease-in gap-2 xl:gap-6 ${open ? 'top-12 bg-[#010427] md:bg-transparent' : 'top-[-510px]'}`}>
                    {
                        NavbarData.Links.map((Link, index) => (
                            <li key={index} className='md:ml-2  md:my-0 my-7 font-normal '>
                                <RouterLink
                                    to={Link.LinkPath}
                                    className='text-white duration-500 hover:text-orange-500'
                                    onClick={() => {
                                        setActiveLink(index)
                                        setOpen(!open)

                                    }}
                                >
                                    <div  className='flex flex-col justify-center text-custom-size-14 sm:leading-10 md:text-xl  items-center'>
                                        {Link?.LinkName}
                                        <div className={`${activeLink === index ? ' bg-[#F7931A] w-full h-[1px] invisible md:visible' : 'w-1 h-[1px] invisible'}`}></div>
                                        <div className={`${activeLink === Link.LinkPath ? ' bg-[#F7931A] w-full h-[1px] invisible md:visible' : 'w-1 h-[1px] invisible'}`}></div>
                                    </div>
                                </RouterLink>
                            </li>
                        ))
                    }

                    <div className='block font-semibold text-center my-7 md:hidden '>

                        <div
                            onClick={() => {
                                if (!isAuthenticated) {
                                    setClickConnectWallet(true);
                                    setOpen(!open)
                                }

                               
                            }}>
                            <GradientButton
                                CustomCss={`hover:opacity-75 w-[150px]  text-xs md:text-base lg:text-base lg:h-[60px] py-2 lg:py-4 px-2`}
                            >{!isAuthenticated ? NavbarData.ButtonText : NavbarData.ButtonTextDisconnet}</GradientButton>
                        </div>
                    </div>

                </ul>
{/* desktop */}
                <div className="w-full  rounded-2xl  flex justify-between max-w-[1200px] tracking-wide items-center py-4 md:py-4 px-2">

                    <div className='flex items-center justify-between px-2 md:justify-start'>
                        <div className='flex items-center justify-around' onClick={()=> navigate("/")} >
                            <img src="./image/valueswap.png" alt="" className='w-28 h-full sm:w-36 cursor-pointer object-contain' />
                            <div className="items-center hidden md:inline-block h-8 ml-2 md:ml-4 lg:ml-6 border-l border-white "></div>
                        </div>


                    </div>
                    <div className='w-[30%] md:w-[70%] flex justify-center md:gap-x-4'>
                        <div className='text-base flex  gap-4   items-center rounded-b-lg  md:w-[100%] px-2'>
                            <ul className={`md:flex md:items-center  md:pb-0 pb-12 hidden md:static rounded-lg left-0 w-full md:w-auto md:pl-0  transition-all duration-500 ease-in gap-2 xl:gap-6 ${open ? 'top-12 bg-[#010427] md:bg-transparent' : 'top-[-490px]'}`}>
                                {
                                    NavbarData.Links.map((Link, index) => (
                                        <li key={index} id='navLink' className='md:ml-2  md:my-0 my-7 font-normal '>
                                            <RouterLink
                                                to={Link.LinkPath}
                                                className='text-white duration-500 '
                                                onClick={() => {
                                                    setActiveLink(index)
                                                    setOpen(!open)

                                                }}
                                            >
                                                <div  className='flex flex-col justify-center text-custom-size-14 sm:leading-10 md:text-xl  items-center  '>
                                                    {Link?.LinkName}
                                                    {/* <div className={ 'bg-[#F7931A] w-full h-[1px] invisible hover:visible' }></div> */}
                                                    <div className={`${activeLink === Link.LinkPath ? ' bg-[#F7931A] w-full h-[1px] hover:invisible  invisible md:visible' : 'w-1 h-[1px] hover:invisible invisible'}`}></div>
                                                </div>
                                            </RouterLink>
                                        </li>
                                    ))
                                }
                                

                                <div className='block font-semibold text-center my-7 md:hidden '>

                                    <div
                                        onClick={() => {
                                            if (NavbarData.ButtonText === 'Connect') {
                                                setClickConnectWallet(true);
                                            }

                                            if (NavbarData.ButtonText === 'Explore Pools') {
                                                navigate('/valueswap/pool')
                                            }
                                        }}>
                                        <BorderGradientButton
                                            CustomCss={`hover:opacity-75 w-[150px]  text-xs md:text-base lg:text-base lg:h-[60px] py-2 lg:py-4 px-2`}
                                        >{NavbarData.ButtonText}</BorderGradientButton>
                                    </div>
                                </div>

                            </ul>

                        </div>

                        {/* drop down Network*/}
                        {/* {location.pathname === "/valueswap" && <div className="relative inline-block ">
                            <div
                                className="flex items-center p-2 rounded-md cursor-pointer  gap-x-2"
                                onClick={() => document.getElementById('options-container').classList.toggle('hidden')}
                            >
                                <div className="flex items-center">
                                    {selectedOption && <img src={selectedOption?.img} alt={selectedOption?.label} className="w-6 h-6 mr-2" />}
                                    <span className='hidden md:inline-block'>{selectedOption?.label}</span>
                                </div>
                                <svg className="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </div>
                            <div id="options-container" className="absolute min-w-32 z-10 md:mt-1 p-2 top-12 md:top-16 right-[-2rem] bg-[#05071D] border border-gray-300 rounded-md shadow-lg hidden md:w-[200%] w-[112%]">
                                <h1 className='text-center '>Select Network</h1>
                                {options && options.map((option) => (
                                    <div
                                        key={option.value}
                                        className="flex items-center p-2 rounded-lg cursor-pointer hover:border hover:border-gray-500 "
                                        onClick={() => {
                                            setSelectedOption(option);
                                            document.getElementById('options-container').classList.add('hidden');
                                        }}
                                    >
                                        <img src={option.img} alt={option.label} className="w-6 h-6 mr-2" />
                                        <span>{option.label}</span>
                                    </div>
                                ))}
                            </div>
                        </div>} */}
                    </div>
                    {/* //// */}
                    <div className='hidden pr-1 font-semibold md:my-0 my-7 md:flex md:items-center md:gap-5'>
                        {/* <div className="h-12 border-l border-white"></div> */}
                        <div className='flex items-center '>


                            {!isAuthenticated ? <BorderGradientButton customCss={`bg-[#000711] z-10`}
                            >
                                {
                                    NavbarData.ButtonText === "Connect" ? (
                                        <div

                                        >
                                            {!isAuthenticated && (<div
                                                onClick={() => {
                                                    setClickConnectWallet(true);
                                                }}>
                                                {NavbarData.ButtonText}
                                            </div>)}
                                        </div>
                                    ) : (
                                        <div className=''

                                            onClick={() => {
                                                navigate('/valueswap/pool')
                                            }}>
                                            {NavbarData.ButtonText}
                                        </div>
                                    )
                                }
                            </BorderGradientButton> : <Profile Principal={Principal} principal={principal} isAuthenticated={isAuthenticated} logout={logout}/>}


                        </div>
                    </div>

                    <div onClick={() => setOpen(!open)} className='cursor-pointer md:hidden w-7 h-7'>
                        {open ? <XMarkIcon className='text-white ' /> : <Bars3BottomRightIcon className='text-white' />}
                    </div>
                </div>
            </div>
        </div >
    );
};

export default MobileNavbar;