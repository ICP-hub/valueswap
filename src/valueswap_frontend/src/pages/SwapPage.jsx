import React from 'react'
import Swap from '../Modals/Swap'
import ConnectWallet from '../Modals/ConnectWallet'
import { useSelector } from 'react-redux'
import { useAuth } from '../components/utils/useAuthClient'
const SwapPage = () => {



    return (
        <div>
            
            <div>
                { (<div>
                    <Swap />
                </div>) }
            </div>
        </div>
    )
}

export default SwapPage
