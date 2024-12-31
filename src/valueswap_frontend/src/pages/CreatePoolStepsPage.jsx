import React, { useState } from 'react';
import SelectTokensForPools from '../Modals/poolCreation/SelectTokensForPools';
import SetPoolFees from '../Modals/poolCreation/SetPoolFees';
import InitialLiquidity from '../Modals/poolCreation/InitialLiquidity';
import ArrowBackIcon from '@mui/icons-material/ArrowBack';
const steps = ['Select Tokens for Pools', 'Set Pool Fees', 'Add Initial Liquidity'];

const CreatePoolStepsPage = () => {
    const [activeStep, setActiveStep] = useState(0);
    const isLastStep = activeStep === steps.length - 1;
   const [fixedActiveSetp, setFixedActiveSetp] = useState(0)
   console.log("setFixedActiveSetp", fixedActiveSetp)
    const handleNext = () => {
        console.log("CLicked")
        if (!isLastStep) {
            setActiveStep(current => current + 1);
        }
    };

    const handleStepBack = () => {
        if (activeStep > 0) {
            setActiveStep(current => current - 1);
        }
    };



    const getStepContent = (step) => {
        switch (step) {
            case 0:
                return <SelectTokensForPools handleNext={handleNext}  setFixedActiveSetp ={setFixedActiveSetp}/>;
            case 1:
                return <SetPoolFees handleNext={handleNext} setFixedActiveSetp ={setFixedActiveSetp}/>;
            case 2:
                return <InitialLiquidity />;
            default:
                return 'Unknown step';
        }
    };

    return (
        <div className=" md:mx-6 my-10 relative">
             <button onClick={handleStepBack} className="mb-4 p-2 border-[1px] rounded-full inline-block lg:hidden ml-4 sm:ml-10 md:ml-12">
                <ArrowBackIcon/>
            </button>
            <div className=" lg:flex-row flex-col py-2 justify-around hidden lg:flex max-w-[1200px] mx-auto font-gilroy">
                {steps.map((label, index) => (
                       <div key={index} className= {`flex w-full items-center justify-center md:gap-4`} onClick={() => setActiveStep(index <= fixedActiveSetp ? index : fixedActiveSetp)}>
                        <div className={`relative flex aspect-square md:px-6 items-center justify-center rounded-full bg-gray-900 
                             ${activeStep === index  ? "shadow-lg ring-1 ring-gray-700/50 before:-inset-1 before:rounded-full before:bg-white/5 before:blur-md opacity-100":"ring-0 opacity-80"}`}>
                            
                            {activeStep === index &&  <div className="absolute -inset-0.5 rounded-full bg-gradient-to-r from-blue-100/20 to-white/20 opacity-75 blur-md group-hover:opacity-100" />}
                            <span className={`lg:text-4xl md:text-3xl text-2xl text-white ${activeStep == index ? "font-semibold" : "font-normal"}`} >
                                {index+ 1}
                            </span>
                        </div>
                       <p className="text-lg">{label}</p>
                       <hr className="border-2 w-1/4 pr-6" />
                     </div>
                ))}
            </div>
            <div className=''>
                {/* <div className="text-lg font-semibold mb-2">Step {activeStep + 1}</div> */}
                <div className='font-gilroy'>
                    {getStepContent(activeStep)}
                </div>
                {/* <div className="flex mt-4">
                    <button className={`mr-2 p-5 rounded-full bg-[#8D4C00] ${activeStep === 0 ? 'opacity-50 cursor-not-allowed' : 'hover:opacity-50'}`} disabled={activeStep === 0} onClick={handleBack}>
                        <MoveLeft size={30} />
                    </button>
                    <div className="flex-grow"></div>
                    {isLastStep ? (
                        <button className="p-5 rounded-full bg-[#8D4C00] opacity-50 cursor-not-allowed">
                            <MoveRight size={30} />
                        </button>
                    ) : (
                        <button className="p-5 rounded-full bg-[#8D4C00] hover:opacity-50" onClick={handleNext}>
                            <MoveRight size={30} />
                        </button>
                    )}
                </div> */}
            </div>
        </div>
    );
}

export default CreatePoolStepsPage;
