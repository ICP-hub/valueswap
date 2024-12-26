import Container from "../components/onWithdrawPageComponents/Container";
import { ArrowLeftIcon } from "lucide-react";
const OnWithDraw=()=>{
    return(
        <main className="min-h-screen h-auto flex flex-col justify-center items-center relative">
            <span className="hover:cursor-pointer active:cursor-pointer absolute top-4 left-1/4">
                <ArrowLeftIcon color="#fff" size={26}/>
            </span>
            <Container/>
        </main>
    )
}

export default OnWithDraw;