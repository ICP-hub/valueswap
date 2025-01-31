export default function LiquidityInfo() {
    return (
      <div className="relative min-h-screen bg-[#020817] overflow-hidden">
        <div
          className="absolute inset-0 z-0 bg-cover bg-center"
          style={{
            backgroundImage:
              "url('/image/globe.svg')",
            filter: "brightness(0.4) blur(3px)",
            backgroundSize:'contain',
            backgroundRepeat:'no-repeat',
            backgroundPosition:'center center'
          }}
        />
  
        <div className="relative z-10 p-4 sm:p-6 md:p-8 font-gilroy">
          <div className="max-w-7xl mx-auto relative" style={{ height: "90vh" }}>
            {/* Total Liquidity Card - Largest */}
            <div
              className="absolute left-[5%] sm:left-[10%] top-[5%] sm:top-[10%] w-[80%] sm:w-[60%] md:w-[50%] lg:w-[40%] xl:w-[35%] transition-transform duration-300 hover:scale-105"
              style={{ zIndex: 4 }}
            >
              <div className="relative backdrop-blur-md bg-white/5 rounded-lg p-4 sm:p-6 md:p-8 border border-white/10 shadow-[0_0_25px_rgba(51,90,255,0.15)]">
                <div className="text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold text-white tracking-tight">
                  1.58B
                </div>
                <div className="text-xs sm:text-sm text-gray-400 mb-1 sm:mb-2">Total Liquidity</div>
              </div>
            </div>
  
            {/* Swap Volume Card - Medium */}
            <div
              className="absolute right-[5%] sm:right-[15%] top-[30%] sm:top-[20%] w-[70%] sm:w-[50%] md:w-[40%] lg:w-[30%] xl:w-[25%] transition-transform duration-300 hover:scale-105"
              style={{ zIndex: 3 }}
            >
              <div className="relative backdrop-blur-md bg-white/5 rounded-lg p-4 sm:p-6 border border-white/10 shadow-[0_0_20px_rgba(51,90,255,0.15)]">
                <div className="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold text-white tracking-tight">
                  250.59M
                </div>
                <div className="text-xs sm:text-sm text-gray-400 mb-1 sm:mb-2">Swap Vol</div>
              </div>
            </div>
  
            {/* Liquidity Providers Card - Medium Small */}
            <div
              className="absolute left-[10%] sm:left-[25%] bottom-[30%] sm:bottom-[35%] w-[60%] sm:w-[45%] md:w-[35%] lg:w-[28%] xl:w-[22%] transition-transform duration-300 hover:scale-105"
              style={{ zIndex: 2 }}
            >
              <div className="relative backdrop-blur-md bg-white/5 rounded-lg p-4 sm:p-6 border border-white/10 shadow-[0_0_15px_rgba(51,90,255,0.15)]">
                <div className="text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold text-white tracking-tight">
                  170.30K
                </div>
                <div className="text-xs sm:text-sm text-gray-400 mb-1 sm:mb-2">Liquidity Providers</div>
              </div>
            </div>
  
            {/* Total Pools Card - Smallest */}
            <div
              className="absolute right-[15%] sm:right-[30%] bottom-[10%] sm:bottom-[20%] w-[50%] sm:w-[35%] md:w-[25%] lg:w-[20%] xl:w-[18%] transition-transform duration-300 hover:scale-105"
              style={{ zIndex: 1 }}
            >
              <div className="relative backdrop-blur-md bg-white/5 rounded-lg p-3 sm:p-4 border border-white/10 shadow-[0_0_15px_rgba(51,90,255,0.15)]">
                <div className="text-lg sm:text-xl md:text-2xl lg:text-3xl font-bold text-white tracking-tight">7.5K</div>
                <div className="text-xs sm:text-sm text-gray-400 mb-1 sm:mb-2">Total Pools</div>
              </div>
            </div>
  
            {/* Floating Elements */}
            <div className="absolute inset-0 pointer-events-none">
              <div className="absolute top-1/4 left-1/3 w-1 h-1 sm:w-2 sm:h-2 bg-blue-500 rounded-full animate-ping" />
              <div
                className="absolute top-2/3 right-1/4 w-1 h-1 sm:w-2 sm:h-2 bg-purple-500 rounded-full animate-ping"
                style={{ animationDelay: "1s" }}
              />
              <div
                className="absolute bottom-1/4 left-1/2 w-1 h-1 sm:w-2 sm:h-2 bg-cyan-500 rounded-full animate-ping"
                style={{ animationDelay: "1.5s" }}
              />
            </div>
          </div>
        </div>
      </div>
    )
  }
  
  