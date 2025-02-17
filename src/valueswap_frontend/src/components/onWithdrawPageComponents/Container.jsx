import GradientButton from '../../buttons/GradientButton';
import Slider from './Slider';
import TokenDisplay from './TokenDisplay';
import { useParams } from 'react-router-dom';
import { usePoolData } from '../../hooks/usePoolData';
const Container = () => {
    const {id} = useParams()
    const {tokens, loading} = usePoolData(id)
    console.log("tokens", tokens)
    if(loading){
        return(
            <div className="bg-transparent dividie-y divide-white divide-opacity-50 p-2
            md:min-w-[500px] sm:min-w-[450px] min-w-full flex flex-col md:justify-center justify-between space-y-2">
                <div className='backdrop-blur-[32px] md:p-6 sm:p-4 p-2 rounded-lg border border-white border-opacity-75'>
                    <Slider/>
                </div>
                {/* <TokenDisplay/> */}
                <GradientButton CustomCss="w-full">
                    Withdraw
                </GradientButton>
            </div>
        )
    }
    return(
        <div className="bg-transparent dividie-y divide-white divide-opacity-50 p-2
        md:min-w-[500px] sm:min-w-[450px] min-w-full flex flex-col md:justify-center justify-between space-y-2">
            <div className='backdrop-blur-[32px] md:p-6 sm:p-4 p-2 rounded-lg border border-white border-opacity-75'>
                <Slider/>
            </div>
            <TokenDisplay tokens={tokens}/>
            <GradientButton CustomCss="w-full">
                Withdraw
            </GradientButton>
        </div>
    )
};

export default Container;