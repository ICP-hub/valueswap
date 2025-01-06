import React, { useEffect, useState } from 'react'
import GradientButton from '../../buttons/GradientButton'
// import { AllPool } from '../../TextData';
import { useNavigate } from 'react-router-dom'
import ArrowDownwardIcon from '@mui/icons-material/ArrowDownward'
import ImportExportIcon from '@mui/icons-material/ImportExport';
import Skeleton, { SkeletonTheme } from 'react-loading-skeleton'
import 'react-loading-skeleton/dist/skeleton.css'
import { useAuth, useAuthClient } from '../utils/useAuthClient'
import BorderGradientButton from '../../buttons/BorderGradientButton'
import {portfolioSampleData} from "../../TextData"
import KeyboardArrowRightIcon from '@mui/icons-material/KeyboardArrowRight';
import KeyboardArrowLeftIcon from '@mui/icons-material/KeyboardArrowLeft';
import KeyboardDoubleArrowLeftIcon from '@mui/icons-material/KeyboardDoubleArrowLeft';
import KeyboardDoubleArrowRightIcon from '@mui/icons-material/KeyboardDoubleArrowRight';
const PortfolioDataComponent = () => {
  const [allDataInPool, setAllDataInPool] = useState([])
  const [displayCount, setDisplayCount] = useState(0)
  const [buttonVisible, setButtonVisibility] = useState(true)
  const [activeSort, setActiveSort] = useState()
  const [isAscending, setIsAscending] = useState(true)
  const { backendActor, principal } = useAuth()
  const [poolName, setPoolName] = useState()
  const [itemsPerPage] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);
  const { isAuthenticated } = useAuth()

  //  const listOfPool = [];
  useEffect(() => {
    const userPools = async () => {
      const AllPool = await backendActor?.get_user_pools_with_lp(principal)
      console.log(" get_user_pools_with_lp", AllPool)

      setPoolName(AllPool)
      for (let i = 0; i < AllPool.length; i++) {
        // console.log('AllPool', AllPool[i][0])
        const poolData = await backendActor?.get_specific_pool_data(
          AllPool[0][i][0]
        )
        console.log("poolData", poolData)
        let specificData = poolData.Ok[0]
       
        setAllDataInPool(prev => [...prev, {...specificData, Lp: AllPool[0][i][1]}])
      }
      setDisplayCount(Math.min(5, AllPool.length))
    }
    userPools()
  }, [backendActor])
  console.log('allDataInPool', allDataInPool)
  useEffect(() => {
    if (allDataInPool?.length < 6) {
      setButtonVisibility(false)
    }
  }, [allDataInPool])

  const sortBalance = () => {
    const sortedTableData = [...allDataInPool].sort((a, b) => {
      const aValue =
        typeof a.PoolMetaData.Balance === 'string'
          ? parseFloat(a.PoolMetaData.Balance.replace(/[\$,]/g, ''))
          : a.PoolMetaData.Balance
      const bValue =
        typeof b.PoolMetaData.Balance === 'string'
          ? parseFloat(b.PoolMetaData.Balance.replace(/[\$,]/g, ''))
          : b.PoolMetaData.Balance
      return isAscending ? bValue - aValue : aValue - bValue
    })

    setAllDataInPool({ ...allDataInPool, TableData: sortedTableData })
    setIsAscending(!isAscending)
  }

  const sortPoolVolume = () => {
    const sortedTableData = [...allDataInPool.TableData].sort((a, b) => {
      const aVolume =
        typeof a.PoolMetaData.PoolValue === 'string'
          ? parseFloat(a.PoolMetaData.PoolValue.replace(/[\$,]/g, ''))
          : a.PoolMetaData.PoolValue
      const bVolume =
        typeof b.PoolMetaData.PoolValue === 'string'
          ? parseFloat(b.PoolMetaData.PoolValue.replace(/[\$,]/g, ''))
          : b.PoolMetaData.PoolValue
      return isAscending ? bVolume - aVolume : aVolume - bVolume
    })
    setAllDataInPool({ ...allDataInPool, TableData: sortedTableData })
    setIsAscending(!isAscending)
  }

  const sortApr = () => {
    const sortedTableData = [...allDataInPool.TableData].sort((a, b) => {
      const aprA = a.PoolMetaData.APRstart
      const aprB = b.PoolMetaData.APRend

      if (aprA && aprB) {
        if (isAscending) {
          return aprB - aprA
        } else {
          return aprA - aprB
        }
      }
      return 0
    })
    setAllDataInPool({ ...allDataInPool, TableData: sortedTableData })
    setIsAscending(!isAscending)
  }

  const sortingConditional = poolIndex => {
    if (poolIndex === 1) {
      sortBalance()
      setActiveSort(poolIndex)
    } else if (poolIndex === 2) {
      sortPoolVolume()
      setActiveSort(poolIndex)
    } else if (poolIndex === 3) {
      sortApr()
      setActiveSort(poolIndex)
    }
  }

  // pagination logic
  const  totalPages = Math.ceil(allDataInPool?.length / itemsPerPage);

  const currentItems = allDataInPool?.slice(
    (currentPage -1) * itemsPerPage, 
    currentPage * itemsPerPage
  )


  const handleNextPage = () => {
    if(currentPage < totalPages) setCurrentPage(currentPage + 1);
  }

  const handlePrevPage = () => {
    if(currentPage > 1) setCurrentPage(currentPage - 1);
  }

  const handleFirstPage = () => {
    setCurrentPage(1);
  }

  const handleLastPage = () => {
    setCurrentPage(totalPages);
  }

  const navigate = useNavigate()
  const Headings = ['Pool name', 'Staking', 'TVL', 'Volume(24h)', "LP", 'APR']
  return (
    <div className='max-w-[1200px] mx-auto  relative'>
      <div className='flex justify-between mt-8 px-8 mx-auto'>
        <div className='bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-10 border  border-[#FFFFFF66] rounded-2xl w-[48%] py-8 text-center'>
          <h3 className=''>My liquidity</h3>
          <h1 className='text-3xl font-semibold tracking-wide'>$0.00</h1>
        </div>
        <div className='bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-10 border  border-[#FFFFFF66] rounded-2xl w-[48%] py-8 text-center'>
          <h3>Claimable incentives</h3>
          <h1 className='text-3xl font-semibold tracking-wide'>$0.00</h1>
        </div>
      </div>
      <div className='w-full  text-white  px-8 mx-auto pb-8'>
        <div className='flex justify-between  p-2 pb-6 pt-6 rounded-t-lg mx-auto'>
          {/* search box */}
          <div className='flex w-full gap-x-2 justify-center items-center'>
            <div className='relative w-full '>
              <div className='absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none'>
                <svg
                  className='w-5 h-5 text-gray-400'
                  fill='none'
                  stroke='currentColor'
                  viewBox='0 0 24 24'
                  xmlns='http://www.w3.org/2000/svg'
                >
                  <path
                    strokeLinecap='round'
                    strokeLinejoin='round'
                    strokeWidth='2'
                    d='M21 21l-4.35-4.35M17 10.5A6.5 6.5 0 104 10.5a6.5 6.5 0 0013 0z'
                  ></path>
                </svg>
              </div>

              <input
                type='text'
                placeholder='Search...'
                className='w-full py-2 pl-10 pr-4 bg-transparent rounded-lg shadow-inner outline-none  text-gray-400 placeholder-gray-400 border border-[#FFFFFF66]'
                // value={filterData}
                // onChange={(e) => setFilterData(e.target.value)}
              />
            </div>
            <BorderGradientButton
              customCss={`bg-[#000711]  w-[150px]  text-xs md:text-base lg:text-base lg:h-[45px]   px-2`}
            >
              Filters
            </BorderGradientButton>
            <div
              className='mr-4'
              onClick={() => navigate('/valueswap/pool/create-pool/steps')}
            >
              <GradientButton
                CustomCss={`hover:opacity-75 text-xs md:text-base lg:text-base h-[45px] w-[120px] py-2 lg:py-4`}
              >
                Create Pool
              </GradientButton>
            </div>
          </div>
        </div>
        <div className='flex flex-col font-gilroy min-h-[30%]  bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-10 border  border-[#FFFFFF66] rounded-2xl '>
          <div className='-my-2 overflow-x-auto'>
            <div className='inline-block min-w-full py-2 align-middle'>
              {allDataInPool.Ok?.length <= 0 ? (
                <div> No Pool found ! </div>
              ) : (
                <div className='overflow-hidden shadow ring-1 ring-black ring-opacity-5'>
                  <SkeletonTheme
                    baseColor='#1f2029'
                    highlightColor='#2b2b2b'
                    borderRadius='0.5rem'
                  >
                    <table className='min-w-full min-h-1/2'>
                      <thead>
                        <tr>
                          {Headings?.map((heading, index) => (
                            <th
                              scope='col'
                              key={index}
                              className={`py-7    md:pr-0 text-center  text-sm md:text-base lg:text-base  font-bold text-white ${
                                heading == 'Pool name' ? 'w-7/12' : ''
                              } `}
                            >
                              <span
                                className={`flex  items-center ml-4 cursor-pointer ${index === activeSort ? "text-[#F7931A]": ""}`}
                                onClick={() => sortingConditional(index)}
                              >
                                {heading}
                                {index === activeSort ? (
                                  <ArrowDownwardIcon sx={{ color: '' }}  fontSize='small'/>
                                ) : (
                                  <ImportExportIcon fontSize='small'/>
                                )}
                              </span>
                            </th>
                          ))}
                        </tr>
                      </thead>
                      <tbody>
                        {isAuthenticated ? (
                          //  allDataInPool
                          !currentItems ? (
                            Array.from({ length: 3 }).map((_, index) => (
                              <tr key={index}>
                                <td className='px-3 py-4 text-sm text-center text-white whitespace-nowrap md:text-base'>
                                  <Skeleton height={30} />
                                </td>
                                <td className='px-3 py-4 text-sm text-center text-white whitespace-nowrap md:text-base'>
                                  <Skeleton height={30} />
                                </td>
                                <td className='px-3 py-4 text-sm text-center text-white whitespace-nowrap md:text-base'>
                                  <Skeleton height={30} />
                                </td>
                                <td className='px-3 py-4 text-sm text-center text-white whitespace-nowrap md:text-base'>
                                  <Skeleton height={30} />
                                </td>
                              </tr>
                            ))
                          ) : (
                            currentItems?.map((Poolinfo, index) => (
                              <tr
                                key={index}
                                className='hover:bg-[#546093] rounded-xl cursor-pointer'
                                onClick={() => {
                                  navigate(
                                    `/valueswap/portfolio/pool-info/${
                                      Poolinfo.pool_name || index
                                    }`
                                  )
                                }}
                              >
                                <td className='min-w-80 whitespace-nowrap my-4 text-sm md:text-base font-medium text-white flex flex-wrap items-center gap-5 justify-start ml-4'>
                                  {Poolinfo?.pool_data.map((pool, indx) => (
                                    <span
                                      key={indx}
                                      className='flex items-center justify-center gap-x-1 cursor-pointer border-[1px] border-[#FFFFFF66] rounded-2xl py-1 px-2'
                                    >
                                      <img
                                        src={pool.image}
                                        alt=''
                                        className='w-4 h-4'
                                      />
                                     {Poolinfo?.pool_data.length < 4 ? <span className='font-bold'>{pool.token_name}</span> :""}
                                      <span className='text-sm flex items-center justify-center'>{Number(pool.weight )} %</span>
                                    </span>
                                  ))}
                                </td>
                                <td className='whitespace-nowrap py-4 pl-3 text-center text-sm md:text-base font-medium pr-2'>
                                  $
                                  {(() => {
                                    const value = Poolinfo?.pool_data?.reduce(
                                      (sum, item) => sum + BigInt(item.value),
                                      BigInt(0)
                                    )
                                    return value?.toLocaleString('en-US')
                                  })()}
                                </td>
                                <td className='whitespace-nowrap px-3 py-4 text-sm md:text-base text-white text-center'>
                                  {(() => {
                                    const totalBalance =
                                      Poolinfo?.pool_data?.reduce(
                                        (sum, item) =>
                                          sum + BigInt(item.balance),
                                        BigInt(0)
                                      )
                                    return totalBalance?.toLocaleString('en-US')
                                  })()}
                                </td>
                                <td className='whitespace-nowrap px-3 py-4 text-sm md:text-base text-white text-center'>
                                  1% - 2%
                                </td>
                                <td className='whitespace-nowrap px-3 py-4 text-sm md:text-base text-white text-center'>
                                  {(Number(Poolinfo?.Lp)/100000000).toFixed(4)}
                                </td>
                              </tr>
                            ))
                          )
                        ) : (
                          <tr>
                            <td colSpan={4} className='text-center text-white'>
                              <GradientButton
                                CustomCss={`hover:opacity-75 text-xs md:text-base lg:text-base h-[45px] w-[120px] py-2 lg:py-4`}
                              >
                                Connect wallet
                              </GradientButton>
                            </td>
                          </tr>
                        )}
                      </tbody>
                    </table>
                  </SkeletonTheme>
                  {(
                  <div className='mt-4 px-4 text-center pb-4'>
                    <div className="flex gap-x-4 ">
                    <KeyboardDoubleArrowLeftIcon
                    onClick={handleFirstPage}
                    className='cursor-pointer'/>
                    <div className='flex gap-4 '>
                      <KeyboardArrowLeftIcon 
                      onClick={handlePrevPage}
                      className='cursor-pointer'/>
                      <div className='flex gap-x-2'>
                      <span className='text-gray-400'>
                      Page {currentPage} of {totalPages}
                    </span>
                      </div>
                      <KeyboardArrowRightIcon 
                       onClick={handleNextPage}
                      className='cursor-pointer'/>
                    </div>
                    <KeyboardDoubleArrowRightIcon 
                     onClick={handleLastPage}
                    className='cursor-pointer'/>
                    </div>
                  </div>
                )}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default PortfolioDataComponent
