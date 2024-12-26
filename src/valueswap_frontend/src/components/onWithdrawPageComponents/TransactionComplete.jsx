import { Check } from "lucide-react"
const TransactionComplete = () => {
    return (
        <div className="flex flex-col items-center justify-center gap-4 p-6 text-center font-cabin h-full">
            <div className="rounded-full bg-green-500 p-3">
                <Check className="h-12 w-12 text-white" strokeWidth={3} />
            </div>
            <div className="space-y-2">
                <h2 className="text-xl font-semibold">
                    Transaction Completed Successfully!
                </h2>
                <p className="text-sm text-muted-foreground">
                    You remove liquidity from on of your pools
                </p>
            </div>
        </div>
    )
}

export default TransactionComplete