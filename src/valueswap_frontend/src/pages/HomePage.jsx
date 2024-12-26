import React, { useState, useEffect } from 'react'
import SwapPage from './SwapPage'
import TransactionPage from './TransactionPage'
import PoolPage from './PoolPage'
import PortfolioPage from './PortfolioPage'
import { Routes, Route } from 'react-router-dom'
import Faucet from './Faucet'
import OnWithDraw from './OnWithdraw'

const HomePage = () => {


    return (
        <div className='w-full '>
            <Routes>
                <Route path="/" element={<SwapPage/>} />
                <Route path="/transaction-successfull" element={<TransactionPage />} />
                <Route path="/portfolio/*" element={<PortfolioPage />} />
                <Route path="/pool/*" element={<PoolPage />} />
                <Route path='/faucet' element={<Faucet/>}/>
                <Route path="/on-withdraw" element={<OnWithDraw/>} />
            </Routes>
        </div>
    )
}

export default HomePage
