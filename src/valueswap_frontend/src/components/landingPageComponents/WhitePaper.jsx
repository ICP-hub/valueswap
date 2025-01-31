import React from 'react'
import GradientButton from '../../buttons/GradientButton'
import { LandingPageNavbarData as NavbarData } from '../../TextData'
import { LandingPageData } from '../../TextData'
const whitePaper = () => {
    return (
        <div
        className='mt-32 h-full text-center min-h-80 mx-auto mb-24 space-y-4' id={`${NavbarData.Links[3].LinkId}`}>

            <div className="w-full max-w-[1200px] flex flex-col md:flex-row justify-around items-center mx-auto gap-4">
            <section className="shadow-[0_0_25px_rgba(51,90,255,0.15)] px-8 py-6 rounded-lg bg-linear-to-r from-[#C0D9FF] to-[#D9D9D9] border border-1 border-[#C0D9FF] ">
                <h3 className="text-3xl font-medium text-white mb-6">
                Our <span className="text-[#FF8A00]">Innovative</span> techniques
                </h3>
                <p className="text-gray-300 text-lg mb-8 leading-relaxed">
                By integrating Balancer-like liquidity pool techniques, our decentralized exchange offers unparalleled asset
                handling, empowering users with optimized trading and portfolio
                </p>
                <GradientButton>Get Details</GradientButton>
            </section>

            <section className="shadow-[0_0_25px_rgba(51,90,255,0.15)] px-8 py-6 rounded-lg bg-linear-to-r from-[#C0D9FF] to-[#D9D9D9] border border-1 border-[#C0D9FF] ">
                <h3 className="text-3xl font-medium text-white mb-6">
                <span className="text-[#FF8A00]">Unbeatable </span> Rates
                </h3>
                <p className="text-gray-300 text-lg mb-8 leading-relaxed">
                Enjoy the most competitive trading fees and benefit from our tiered 
                liquidity provider rewards that incentivize deep liquidity provision and fair trading conditions
                </p>
                <GradientButton>Get Details</GradientButton>
            </section>
            </div>
           

            <div 
            style={{backgroundImage : "url('/image/infinity.svg')"}}
            className='shadow-[0_0_25px_rgba(51,90,255,0.15)] whitepaper w-full flex flex-col items-end mx-auto max-w-[1200px] gap-8 md:gap-y-12 py-12 md:py-20 bg-linear-to-r from-[#C0D9FF] to-[#D9D9D9] border border-1 border-[#C0D9FF] rounded-lg h-4/6'>

                <img src="/image/astro-new.png" alt="astro" style={{display:'none'}}/>
                <div className="flex flex-col items-start w-[60%] gap-2">
                    <div className='font-gilroy font-bold md:font-medium md:text-6xl text-2xl'>
                        {LandingPageData.WhitePaperText.Heading}
                    </div>

                    <div className='md:text-xl font-normal font-gilroy leading-6 max-w-7xl text-left'>
                        {LandingPageData.WhitePaperText.Description}
                    </div>

                    <div>
                        <GradientButton>{LandingPageData.WhitePaperText.ButtonText}</GradientButton>
                    </div>
                </div>

            </div>
        </div>
    )
}

export default whitePaper
