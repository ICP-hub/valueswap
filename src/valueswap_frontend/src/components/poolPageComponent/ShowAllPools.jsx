import React, { useState, useEffect } from 'react';
import { poolsSvg } from './PoolPageComponentsSvg';
import GradientButton from '../../buttons/GradientButton';
import { useNavigate } from 'react-router-dom';
import ArrowDownwardIcon from '@mui/icons-material/ArrowDownward';
import ArrowUpwardIcon from '@mui/icons-material/ArrowUpward';
import Skeleton, { SkeletonTheme } from 'react-loading-skeleton';
import 'react-loading-skeleton/dist/skeleton.css';
import { useAuth } from "../utils/useAuthClient";
import { valueswap_backend } from '../../../../declarations/valueswap_backend';

const ShowAllPools = () => {
  const [allDataInPool, setAllDataInPool] = useState(null);
  const [isAscending, setIsAscending] = useState(true); // Tracks sorting order
  const [activeSort, setActiveSort] = useState(null); // Tracks which column is being sorted
  const [displayCount, setDisplayCount] = useState(10); // Tracks how many rows to display
  const { backendActor } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    const fetchPoolData = async () => {
      try {
        const AllPoolsData = await valueswap_backend.get_pool_data();
        setAllDataInPool(AllPoolsData); // Set the fetched data into state
      } catch (error) {
        console.error("Error fetching pool data", error); // Log any errors
      }
    };
    fetchPoolData();
  }, [backendActor]);

  // Sorting logic for PoolValue
  const sortValue = () => {
    if (!allDataInPool?.TableData) return;
    const sortedTableData = [...allDataInPool.TableData].sort((a, b) => {
      const aValue = typeof a.PoolValue === 'string' ? parseFloat(a.PoolValue.replace(/[\$,]/g, '')) : a.PoolValue;
      const bValue = typeof b.PoolValue === 'string' ? parseFloat(b.PoolValue.replace(/[\$,]/g, '')) : b.PoolValue;
      return isAscending ? aValue - bValue : bValue - aValue;
    });
    setAllDataInPool({ ...allDataInPool, TableData: sortedTableData });
    setIsAscending(!isAscending); // Toggle sorting order
  };

  // Sorting logic for TotalVolume
  const sortTotalVolume = () => {
    if (!allDataInPool?.TableData) return;
    const sortedTableData = [...allDataInPool.TableData].sort((a, b) => {
      const aVolume = typeof a.TotalVolume === 'string' ? parseFloat(a.TotalVolume.replace(/[\$,]/g, '')) : a.TotalVolume;
      const bVolume = typeof b.TotalVolume === 'string' ? parseFloat(b.TotalVolume.replace(/[\$,]/g, '')) : b.TotalVolume;
      return isAscending ? aVolume - bVolume : bVolume - aVolume;
    });
    setAllDataInPool({ ...allDataInPool, TableData: sortedTableData });
    setIsAscending(!isAscending);
  };

  // Parsing APR values from string
  const parseAPR = (aprString) => {
    const matches = aprString.match(/(\d+\.\d+)% - (\d+\.\d+)%/);
    if (matches) {
      return {
        min: parseFloat(matches[1]),
        max: parseFloat(matches[2]),
      };
    }
    return null;
  };

  // Sorting logic for APR
  const sortApr = () => {
    if (!allDataInPool?.TableData) return;
    const sortedTableData = [...allDataInPool.TableData].sort((a, b) => {
      const aprA = parseAPR(a.APR);
      const aprB = parseAPR(b.APR);

      if (aprA && aprB) {
        if (isAscending) {
          return aprA.min - aprB.min || aprA.max - aprB.max;
        } else {
          return aprB.min - aprA.min || aprB.max - aprA.max;
        }
      }
      return 0;
    });
    setAllDataInPool({ ...allDataInPool, TableData: sortedTableData });
    setIsAscending(!isAscending);
  };

  // Function to trigger sorting based on the column index
  const sortingConditional = (poolIndex) => {
    if (poolIndex === 1) {
      sortValue();
      setActiveSort(poolIndex);
    } else if (poolIndex === 2) {
      sortTotalVolume();
      setActiveSort(poolIndex);
    } else if (poolIndex === 3) {
      sortApr();
      setActiveSort(poolIndex);
    }
  };

  // Function to show more data (increase display count by 10)
  const handleShowMore = () => {
    setDisplayCount(prevCount => prevCount + 10);
  };

  return (
    <div className='max-w-[1200px] mx-auto h-screen relative'>
      <div className='w-full h-screen text-white mt-4 z-20 sm:px-8 mx-auto absolute'>
        <div className='flex justify-between bg-[#010427] p-2 pb-6 pt-6 rounded-t-lg mx-auto'>
          <div className='flex items-center justify-between gap-4 mx-8 md:gap-16 '>
            <span className='font-medium text-white font-cabin md:text-3xl'>Liquidity Pools</span>
          </div>
          <div className='mr-4' onClick={() => navigate('/dex-swap/pool/create-pool')}>
            <GradientButton CustomCss={`hover:opacity-75 text-xs md:text-base lg:text-base h-[45px] w-[120px] py-2 lg:py-4`}>
              Create Pool
            </GradientButton>
          </div>
        </div>
        <div className='flex flex-col font-cabin bg-[#05071D]'>
          <div className='-my-2 overflow-x-auto'>
            <div className='inline-block min-w-full py-2 align-middle'>
              <div className='overflow-hidden shadow ring-1 ring-black ring-opacity-5'>
                <SkeletonTheme baseColor="#1f2029" highlightColor="#2b2b2b" borderRadius="0.5rem" duration={2}>
                  <table className='min-w-full'>
                    <thead>
                      <tr>
                        {['Pool Name', 'Value', 'Total Volume', 'APR'].map((heading, index) => (
                          <th
                            scope='col'
                            key={index}
                            className='pl-10 pr-3 text-sm font-medium text-center text-white py-7 md:text-base lg:text-xl'
                          >
                            <span className='flex cursor-pointer items-center  gap-2' onClick={() => sortingConditional(index)}>
                              {heading}
                              {index === activeSort ? (
                                isAscending ? (
                                  <ArrowUpwardIcon sx={{ color: '' }} />
                                ) : (
                                  <ArrowDownwardIcon sx={{ color: '' }} />
                                )
                              ) : null}
                            </span>
                          </th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {!allDataInPool
                        ? Array.from({ length: 5 }).map((_, index) => (
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
                        : allDataInPool?.slice(0, displayCount).map((pool, index) => (
                          <tr key={index}>
                            <td className='flex items-center pl-10 pr-3  gap-5 my-4 text-sm font-medium text-white min-w-52 whitespace-nowrap md:text-base'>
                              <span className='flex gap-2'>
                                {/* Placeholder for your token pool images */}
                                <img className='w-10 h-10' src="/image/ckBTC.svg" alt="" />
                              </span>
                              <span className='flex items-center'>
                                <span>{pool[0]}</span> {/* First token name */}
                                <span>:{pool[1][0]?.pool_data?.map((token) => `/${token?.weight}`)}</span> {/* Token weights */}
                              </span>
                            </td>
                            <td className='px-3 py-4 text-sm pl-10 pr-3 text-white whitespace-nowrap md:text-base'>
                              $ {pool.PoolValue?.toLocaleString('en-US') || "0"}
                            </td>
                            <td className='px-3 py-4 text-sm pl-10 pr-3 text-white whitespace-nowrap md:text-base'>
                              $ {pool.TotalVolume?.toLocaleString('en-US') || "0"}
                            </td>
                            <td className='py-4  text-sm font-medium pl-10 pr-3 whitespace-nowrap md:text-base'>
                              {pool.APR || "0"}
                            </td>
                          </tr>
                        ))}
                    </tbody>
                  </table>
                </SkeletonTheme>
                {/* Show More button */}
                {allDataInPool && displayCount < allDataInPool.length && (
                  <div className='mt-4 text-center'>
                    <button className='px-4 py-2 text-white bg-gray-800 rounded-md hover:bg-gray-600' onClick={handleShowMore}>
                      Show More
                    </button>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ShowAllPools;
