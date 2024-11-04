import React, { useEffect, useState } from 'react';
import GradientButton from '../../buttons/GradientButton';
// import { AllPool } from '../../TextData';
import { useNavigate } from 'react-router-dom';
import ArrowDownwardIcon from '@mui/icons-material/ArrowDownward';
import Skeleton, { SkeletonTheme } from 'react-loading-skeleton';
import 'react-loading-skeleton/dist/skeleton.css';
import { useAuth } from '../utils/useAuthClient';

const PortfolioDataComponent = () => {
    const [allDataInPool, setAllDataInPool] = useState([]);
    const [displayCount, setDisplayCount] = useState(0);
    const [buttonVisible, setButtonVisibility] = useState(true);
    const [activeSort, setActiveSort] = useState();
    const [isAscending, setIsAscending] = useState(true);
    const { backendActor, principal } = useAuth()
     const [poolName, setPoolName] = useState([])
    //  const listOfPool = [];
    useEffect(() => {
        const userPools = async () => {
            const AllPool = await backendActor?.get_users_pool(principal)
            setPoolName(AllPool)
            for (let i = 0; i < AllPool.length; i++) {
                console.log("AllPool", AllPool[i][0],)
                const poolData = await backendActor?.get_specific_pool_data(AllPool[i][0])
                let specificData = poolData.Ok[0]
                setAllDataInPool((prev) => [...prev, specificData]);
            }
            setDisplayCount(Math.min(5, AllPool.length));
        }
        userPools();
    }, [backendActor]);
    console.log("allDataInPool", allDataInPool)
    useEffect(() => {
        if (allDataInPool?.length < 6) {
            setButtonVisibility(false);
        }
    }, [allDataInPool]);

    const sortBalance = () => {
        const sortedTableData = [...allDataInPool].sort((a, b) => {
            const aValue = typeof a.PoolMetaData.Balance === 'string' ? parseFloat(a.PoolMetaData.Balance.replace(/[\$,]/g, '')) : a.PoolMetaData.Balance;
            const bValue = typeof b.PoolMetaData.Balance === 'string' ? parseFloat(b.PoolMetaData.Balance.replace(/[\$,]/g, '')) : b.PoolMetaData.Balance;
            return isAscending ? bValue - aValue : aValue - bValue;
        });

        setAllDataInPool({ ...allDataInPool, TableData: sortedTableData });
        setIsAscending(!isAscending);
    };

    const sortPoolVolume = () => {
        const sortedTableData = [...allDataInPool.TableData].sort((a, b) => {
            const aVolume = typeof a.PoolMetaData.PoolValue === 'string' ? parseFloat(a.PoolMetaData.PoolValue.replace(/[\$,]/g, '')) : a.PoolMetaData.PoolValue;
            const bVolume = typeof b.PoolMetaData.PoolValue === 'string' ? parseFloat(b.PoolMetaData.PoolValue.replace(/[\$,]/g, '')) : b.PoolMetaData.PoolValue;
            return isAscending ? bVolume - aVolume : aVolume - bVolume;
        });
        setAllDataInPool({ ...allDataInPool, TableData: sortedTableData });
        setIsAscending(!isAscending);
    };

    const sortApr = () => {
        const sortedTableData = [...allDataInPool.TableData].sort((a, b) => {
            const aprA = a.PoolMetaData.APRstart;
            const aprB = b.PoolMetaData.APRend;

            if (aprA && aprB) {
                if (isAscending) {
                    return aprB - aprA;
                } else {
                    return aprA - aprB;
                }
            }
            return 0;
        });
        setAllDataInPool({ ...allDataInPool, TableData: sortedTableData });
        setIsAscending(!isAscending);
    };

    const sortingConditional = (poolIndex) => {
        if (poolIndex === 1) {
            sortBalance();
            setActiveSort(poolIndex);
        } else if (poolIndex === 2) {
            sortPoolVolume();
            setActiveSort(poolIndex);
        } else if (poolIndex === 3) {
            sortApr();
            setActiveSort(poolIndex);
        }
    };

    const navigate = useNavigate();
    const Headings = ['Token', 'TVL', 'Volume(24h)', 'APR']
    return (
        <div className='max-w-[1200px] mx-auto h-screen relative'>
            <div className='w-full h-screen text-white mt-12 px-8 mx-auto absolute'>
                <div className='flex justify-between bg-[#010427] p-2 pb-6 pt-8 rounded-lg mx-auto'>
                    <div className='flex justify-between items-center mx-2 md:mx-8'>
                        <span className='font-cabin text-xl md:text-3xl font-medium'>My Liquidity Pools</span>
                    </div>
                    <div
                        className='mr-4'
                        onClick={() => {
                            navigate('/valueswap/pool/create-pool/steps')
                        }}>
                        <GradientButton CustomCss={`hover:opacity-75 text-xs md:text-base lg:text-base h-[45px] w-[120px] py-2 lg:py-4`}>
                            Create Pool
                        </GradientButton>
                    </div>
                </div>
                <div className='flex flex-col font-cabin bg-[#05071D]'>
                    <div className='-my-2 overflow-x-auto'>
                        <div className='inline-block min-w-full py-2 align-middle'>
                            {allDataInPool.Ok?.length <= 0 ? <div> No Pool found ! </div> : <div className='overflow-hidden shadow ring-1 ring-black ring-opacity-5'>
                                <SkeletonTheme baseColor="#1f2029" highlightColor="#2b2b2b" borderRadius="0.5rem" duration={2}>
                                    <table className='min-w-full'>
                                        <thead>
                                            <tr >
                                                {Headings?.map((heading, index) => (
                                                    <th
                                                        scope='col'
                                                        key={index}
                                                        className={`py-7 pl-6 pr-10 md:pr-0 text-center text-sm md:text-base lg:text-xl font-medium text-white ${heading == "Token" ?  'w-7/12': ''} `}>
                                                        <span className='flex  items-center ml-4' onClick={() => sortingConditional(index)}>
                                                            {heading}
                                                            {index === activeSort ? <ArrowDownwardIcon sx={{ color: "" }} /> : ""}
                                                        </span>
                                                    </th>
                                                ))}
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {!allDataInPool
                                                ? Array.from({ length: 3 }).map((_, index) => (
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
                                                : allDataInPool?.map((Poolinfo, index) => (
                                                    <tr key={index} className='hover:bg-[#546093] rounded-xl cursor-pointer'
                                                        onClick={() => {
                                                            // const poolName = AllPool[index][0]
                                                            navigate(`/valueswap/portfolio/pool-info/${poolName[index][0]}`);
                                                        }}>

                                                        <td className='min-w-80 whitespace-nowrap my-4 text-sm md:text-base font-medium text-white flex items-center gap-5 justify-start ml-8'>
                                                            {Poolinfo?.pool_data.map((pool, indx) => (
                                                                <span className='flex items-center gap-x-1 cursor-pointer border-2 rounded-2xl py-1 px-2 '>
                                                                    {/* <span key={index} className='bg-[#3D3F47] p-2 rounded-xl'>
                                                                    </span> */}
                                                                        <img src={pool.image} alt="" className='w-6 h-6'  />
                                                                    <span>{pool.token_name}</span>
                                                                    <span>{pool.weight * 100} %</span>
                                                                </span>
                                                            ))}

                                                        </td>
                                                        <td className='whitespace-nowrap py-4 pl-3 text-center text-sm md:text-base font-medium pr-2'>
                                                            {/* {pool?.PoolMetaData?.APRstart}% - {pool?.PoolMetaData?.APRend}% */}
                                                            ${(() => {
                                                                const value = Poolinfo?.pool_data?.reduce(
                                                                    (sum, item) => sum + BigInt(item.value),
                                                                    BigInt(0)
                                                                );
                                                                return  value?.toLocaleString('en-US');
                                                            })()}
                                                        </td>
                                                       
                                                        <td className='whitespace-nowrap px-3 py-4 text-sm md:text-base text-white text-center'>
                                                            {/* $ {pool?.PoolMetaData?.PoolValue.toLocaleString('en-US')} */}
                                                            {(() => {
                                                                const totalBalance = Poolinfo?.pool_data?.reduce(
                                                                    (sum, item) => sum + BigInt(item.balance),
                                                                    BigInt(0)
                                                                );
                                                                return totalBalance?.toLocaleString('en-US');
                                                            })()}
                                                        </td>
                                                        <td className='whitespace-nowrap px-3 py-4 text-sm md:text-base text-white text-center'>
                                                           1% - 2%
                                                        </td>
                                                       
                                                    </tr>

                                                ))}
                                        </tbody>
                                    </table>
                                </SkeletonTheme>
                                <div className='flex justify-center items-center mb-8'>
                                    {buttonVisible && (
                                        <div>
                                            {allDataInPool?.TableData?.length > displayCount && (
                                                <div className='text-center mt-4'>
                                                    <button className='bg-gray-800 text-white px-4 py-2 rounded-md' onClick={() => setDisplayCount(displayCount + 5)}>
                                                        {allDataInPool.SeeMoreButtonText}
                                                    </button>
                                                </div>
                                            )}
                                            {allDataInPool?.TableData?.length <= displayCount && (
                                                <div className='text-center mt-4'>
                                                    <button className='bg-gray-800 text-white px-4 py-2 rounded-md' onClick={() => setDisplayCount(Math.min(5, AllPool.TableData.length))}>
                                                        {allDataInPool.SeeLessButtonText}
                                                    </button>
                                                </div>
                                            )}
                                        </div>
                                    )}
                                </div>
                            </div>}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default PortfolioDataComponent;
