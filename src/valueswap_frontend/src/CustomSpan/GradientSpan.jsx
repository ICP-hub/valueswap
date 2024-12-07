import React from 'react'

const GradientSpan = ({ children }) => {
    return (
        <div>
            <span className='bg-gradient-to-r from-[#FFB45C] via-[#8574EA] to-[#044CD7] text-transparent bg-clip-text font-gilroy'>
                {children}
            </span>
        </div>
    )
}

export default GradientSpan;