export function FloatingIcons() {
    return (
      <div className="absolute inset-0 pointer-events-none overflow-hidden">
        {/* Bitcoin */}
        <div className="absolute left-[10%] bottom-[20%] w-24 h-24 animate-float">
          <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
            <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
              <img src="/image/ckBTC-c.png" alt="ckbtc"/>
            </div>
          </div>
        </div>
  
        {/* Ethereum */}
        <div className="absolute -z-10 right-[15%] bottom-[20%] w-32 h-32 animate-float-delayed">
          <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
            <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
            <img src="/image/ckETH-c.png" alt="cketh"/>
            </div>
          </div>
        </div>

        {/* Tether */}
        <div className="absolute right-[20%] top-[20%] w-16 h-16 animate-float-more-delayed">
          <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
            <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
            <img src="/image/Tether.png" alt="tether"/>
            </div>
          </div>
        </div>
  
        {/* Dollar */}
        <div className="absolute left-[20%] top-[20%] w-16 h-16 animate-float-more-delayed">
          <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
            <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
            <img src="/image/ckUSDC-c.png" alt="ckusdc"/>
            </div>
          </div>
        </div>
      </div>
    )
  }
  
  