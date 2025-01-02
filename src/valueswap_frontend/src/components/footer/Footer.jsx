import React from 'react'
import TwitterIcon from '@mui/icons-material/Twitter';
import GitHubIcon from '@mui/icons-material/GitHub';
import LinkedInIcon from '@mui/icons-material/LinkedIn';
import { BsDiscord, BsYoutube } from 'react-icons/bs';
import NorthEastIcon from '@mui/icons-material/NorthEast';
import { Box } from '@mui/material';
function Footer() {
    return (
        <div className=''>
            <hr className='border-gray-700 ' />
            <div className='grid md:grid-cols-4 grid-cols-2  m-0 gap-y-12 text-white px-4 sm:px-24 md:pr-12   mt-10 pb-14' >

                <div className='space-y-4'>
                    <h1 className='text-lg font-gilroy hover:text-[#C16800] w-1/2'>ValueSwap.io</h1>
                    <ul className='flex flex-col cursor-pointer gap-y-2 font-gilroy'>
                        <li className='hover:text-[#C16800] w-1/2 md:w-1/4'>Home</li>
                        <li className='hover:text-[#C16800] w-1/2 md:w-1/4'>Build</li>
                    </ul>
                </div>
                <div className='space-y-4'>
                    <h1 className='text-lg font-gilroy hover:text-[#C16800] w-1/2 md:w-1/4'>Learn</h1>
                    <ul className='flex flex-col cursor-pointer gap-y-2 font-gilroy'>
                        <li className='hover:text-[#C16800] w-3/4  '>Docs <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /></li>
                        <li className='hover:text-[#C16800] w-3/4 '>Risk <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /></li>
                        <li className='hover:text-[#C16800] w-3/4 '>WhitePaper <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /></li>
                        <li className='hover:text-[#C16800] w-3/4 '>Careers <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /></li>
                        <li className='hover:text-[#C16800] w-3/4 '>Terms of Use <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /></li>
                    </ul>
                </div>
                <div className='space-y-4'>
                    <h1 className='text-lg font-gilroy hover:text-[#C16800] w-1/2 md:w-1/4'>Ecosystem</h1>
                    <ul className='flex flex-col cursor-pointer gap-y-2 font-gilroy'>
                        <li className='hover:text-[#C16800] w-3/4'>Forums <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /></li>
                        <li className='hover:text-[#C16800] w-3/4'>Grants <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /></li>
                        <li className='hover:text-[#C16800] w-3/4'>Brands assets <NorthEastIcon style={{ fontSize: '1rem', paddingBottom: "1px" }} /> </li>
                    </ul>
                </div>
                <div className='space-y-4'>
                    <h1 className='text-lg font-gilroy hover:text-[#C16800] w-1/2 md:w-1/4'>Community</h1>
                    <div className='grid grid-cols-3 pt-4 text-center cursor-pointer md:grid-cols-4 gap-y-4 md:gap-y-5'>

                        <TwitterIcon style={{ fontSize: 24 }} />
                        <GitHubIcon style={{ fontSize: 24 }} />
                        <LinkedInIcon style={{ fontSize: 24 }} />
                        <BsDiscord style={{ fontSize: 24 }} />
                        <BsYoutube style={{ fontSize: 24 }} />

                    </div>
                </div>
            </div>
        </div>
    )
}

export default Footer