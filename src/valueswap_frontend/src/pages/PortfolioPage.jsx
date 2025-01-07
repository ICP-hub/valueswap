import React, { useEffect, useState } from 'react'
import PoolPageBackGround from '../assets/images/PoolPageBackGround.png'
import { Routes, Route } from 'react-router-dom'
import PortfolioDataComponent from '../components/portfolioComponents/PortfolioDataComponent'
import PoolInfo from '../components/portfolioComponents/PoolInfo '
const PortfolioPage = () => {



    return (
        <div className='min-h-screen h-auto'>
        <div className='min-h-screen h-auto'>
                <div className='text-center mt-12'>
                <span className='text-3xl leading-5'>Portfolio</span>
                </div>
         
            <Routes>
                <Route path='/' element={<PortfolioDataComponent />} />
                <Route path='/pool-info/:id' element={<PoolInfo />} />
            </Routes>
        </div>
        </div>
    )
}

export default PortfolioPage
