import { useState } from "react"

function WETHConsent({ checked = false, onCheckedChange}) {
  const [isChecked, setIsChecked] = useState(checked)

  const handleToggle = () => {
    const newValue = !isChecked
    setIsChecked(newValue)
    onCheckedChange?.(newValue)
  }

  return (
    <div 
      className="flex items-center justify-between p-4 rounded-xl border border-orange-500/20 bg-gray-900"
      role="button"
      tabIndex={0}
      onClick={handleToggle}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          handleToggle()
        }
      }}
    >
      <span className="text-white text-sm">Recieve Withdrawal as WETH</span>
      <div className="relative">
        <div 
          className={`w-12 h-6 rounded-full transition-colors duration-200 ease-in-out ${
            isChecked ? "bg-orange-500" : "bg-gray-600"
          }`}
        >
          <div
            className={`absolute top-1 left-1 w-4 h-4 rounded-full bg-white transition-transform duration-200 ease-in-out ${
              isChecked ? "translate-x-6" : "translate-x-0"
            }`}
          />
        </div>
      </div>
    </div>
  )
}

export default WETHConsent