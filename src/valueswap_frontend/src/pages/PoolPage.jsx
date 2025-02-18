import React, { useEffect, useState } from 'react'
import PoolPageBackGround from '../assets/images/PoolPageBackGround.png'
import { Routes, Route } from 'react-router-dom'
import ShowAllPools from '../components/poolPageComponent/ShowAllPools'
import CreatePools from '../components/poolPageComponent/CreatePools'
import CreatePoolStepsPage from './CreatePoolStepsPage'
import { useLocation } from 'react-router-dom'
import AddLiquidity from '../components/poolPageComponent/AddLiquidity'

const PoolPage = () => {
    const location = useLocation();
    const [boxText, setBoxText] = useState('Pool Overview');


    useEffect(() => {
        if (location.pathname.includes("/valueswap/pool/create-pool")) {
            setBoxText('Create Pool');
        } else {
            setBoxText('Pool Overview');
        }
    }, [location])
    return (
        <div className=''>
            <div className='items-center'>
                
                {
                    location.pathname !== '/valueswap/pool/create-pool/steps' ? (
                        <div className='flex justify-center'>
                            {
                                boxText == 'Create Pool' ? (
                                    <div className='my-14 md:m-14 w-11/12 border  text-white p-8 rounded-2xl backdrop-blur-3xl md:w-[60%] lg:w-[47%]'>
                                        <div className='text-start font-fawhkwang font-normal leading-7 text-2xl'>
                                            Rewards For Liquidity Providers
                                        </div>
                                        <div className='border-t-2 my-6 border-[#00308E]'></div>
                                        <span className='font-normal text-base font-gilroy leading-5 italic'>
                                            When you add money to a liquidity pool, you can earn 0.3% of all trades made in that pool. Your earnings are based on how much of the pool you own. Every time someone makes a trade, a small fee is added to the pool, and you can collect your share of those fees whenever you decide to withdraw your money from the pool.
                                            <a href="" className='text-blue-500 underline ml-2'>Learn More</a>
                                        </span>
                                    </div>
                                ) : (
                                    <div className='text-center  font-gilroy  text-white  font-semibold text-2xl mt-6 max-w-72   h-full w-full '>
                                       Liquidity Pools
                                    </div>
                                )
                            }
                        </div>
                    ) : (
                        <div>

                        </div>
                    )
                }
            </div>

            <Routes>
                <Route path='/' element={<ShowAllPools />} />
                <Route path='/addLiquidity/:id' element={<AddLiquidity/>}/>
                <Route path='/create-pool' element={<CreatePools />} />
                <Route path='/create-pool/steps' element={<CreatePoolStepsPage />} />
            </Routes>

        </div>
    )
}

export default PoolPage
