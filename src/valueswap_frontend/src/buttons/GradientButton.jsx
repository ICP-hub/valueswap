import React from 'react';

const GradientButton = ({ CustomCss, children, onClick }) => {

    return (

            <button className={` ${CustomCss} h-[45px]  button-gradient-wrapper mx-auto  text-white text-base font-gilroy rounded-lg py-4 px-12 sm:px-[70px] hover:opacity-50`} onClick={onClick}>
                <span className="flex items-center justify-center p-1 button-gradient-content ">
                    {children}
                </span>
            </button>


    );
};

export default GradientButton;  