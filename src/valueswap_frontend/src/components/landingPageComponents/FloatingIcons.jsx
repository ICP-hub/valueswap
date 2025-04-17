export function FloatingIcons() {
  return (
    <div className="absolute  inset-0 pointer-events-none overflow-hidden ">
      {/* Bitcoin */}
      <div className="absolute -z-10 left-[4%] md:left-[10%] bottom-[27%] md:bottom-[20%] w-16 h-16 md:w-24 md:h-24 animate-float">
        <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
          <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
            <img src="/image/ckBTC-c.png" alt="ckbtc" />
          </div>
        </div>
      </div>

      {/* Ethereum */}
      <div className="absolute -z-10 right-[3%] md:right-[15%] bottom-[26%]  md:bottom-[20%] w-24 h-24 md:w-32 md:h-32 animate-float-delayed">
        <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
          <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
            <img src="/image/ckETH-c.png" alt="cketh" />
          </div>
        </div>
      </div>

      {/* Tether */}
      <div className="absolute right-[7%] md:right-[20%] top-[20%] w-12 h-12 md:w-16 md:h-16 animate-float-more-delayed">
        <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
          <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
            <img src="/image/Tether.png" alt="tether" />
          </div>
        </div>
      </div>

      {/* Dollar */}
      <div className="absolute left-[7%] md:left-[20%] top-[20%] w-12 h-12 md:w-16 md:h-16 animate-float-more-delayed">
        <div className="w-full h-full rounded-full shadow-white shadow-md p-[2px]">
          <div className="w-full h-full rounded-full bg-[#020617] flex items-center justify-center">
            <img src="/image/ckUSDC-c.png" alt="ckusdc" />
          </div>
        </div>
      </div>
    </div>
  );
}
