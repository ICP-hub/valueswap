import React from 'react'

const PoolInfoBox = ({ Heading, Data }) => {
    return (
        <div className='flex flex-col gap-2 justify-center items-start font-gilroy leading-5 '>
            <span className='font-medium text-base opacity-75'>
                {Heading}
            </span>

            <span className='font-medium text-3xl'>
                {Data}
            </span>
        </div>
    )
}

export default PoolInfoBox
