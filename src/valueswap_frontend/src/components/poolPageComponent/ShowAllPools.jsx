import React, { useState, useEffect, useRef } from 'react';
import { poolsSvg } from './PoolPageComponentsSvg';
import GradientButton from '../../buttons/GradientButton';
import { useNavigate } from 'react-router-dom';
import ArrowDownwardIcon from '@mui/icons-material/ArrowDownward';
import ArrowUpwardIcon from '@mui/icons-material/ArrowUpward';
import ImportExportIcon from '@mui/icons-material/ImportExport';
import Skeleton, { SkeletonTheme } from 'react-loading-skeleton';
import 'react-loading-skeleton/dist/skeleton.css';
import { useAuth } from "../utils/useAuthClient";
import { valueswap_backend } from '../../../../declarations/valueswap_backend';

const ShowAllPools = () => {
  const [allDataInPool, setAllDataInPool] = useState(null);
  const [filteredPools, setFilteredPools] = useState(null); // To store filtered results
  const [isAscending, setIsAscending] = useState(true); // Tracks sorting order
  const [activeSort, setActiveSort] = useState(null); // Tracks which column is being sorted
  const [displayCount, setDisplayCount] = useState(10); // Tracks how many rows to display
  const { backendActor } = useAuth();
  const navigate = useNavigate();
  const [filterData, setFilterData] = useState(""); // Search input value
  const ref = useRef();

  // Fetch pool data from backend
  useEffect(() => {
    const fetchPoolData = async () => {
      try {
        const AllPoolsData = await valueswap_backend.get_pool_data();
        setAllDataInPool(AllPoolsData); // Set the fetched data
        setFilteredPools(AllPoolsData); // Initially set filteredPools to all data
      } catch (error) {
        console.error("Error fetching pool data", error);
      }
    };
    fetchPoolData();
  }, [backendActor]);

  // Filter logic based on search input
  useEffect(() => {
    if (filterData.trim() !== "") {
      const filtered = allDataInPool?.filter(poolData => 
        poolData[1][0].pool_data.some(token =>
          token.token_name.toLowerCase().includes(filterData.toLowerCase())
        )
      );
      setFilteredPools(filtered);
    } else {
      setFilteredPools(allDataInPool);
    }
  }, [filterData, allDataInPool]);

  // Sorting logic for PoolValue
  const sortValue = () => {
    if (!filteredPools?.TableData) return;
    const sortedTableData = [...filteredPools.TableData].sort((a, b) => {
      const aValue = typeof a.PoolValue === 'string' ? parseFloat(a.PoolValue.replace(/[\$,]/g, '')) : a.PoolValue;
      const bValue = typeof b.PoolValue === 'string' ? parseFloat(b.PoolValue.replace(/[\$,]/g, '')) : b.PoolValue;
      return isAscending ? aValue - bValue : bValue - aValue;
    });
    setFilteredPools({ ...filteredPools, TableData: sortedTableData });
    setIsAscending(!isAscending); // Toggle sorting order
  };

  // Sorting logic for TotalVolume
  const sortTotalVolume = () => {
    if (!filteredPools?.TableData) return;
    const sortedTableData = [...filteredPools.TableData].sort((a, b) => {
      const aVolume = typeof a.TotalVolume === 'string' ? parseFloat(a.TotalVolume.replace(/[\$,]/g, '')) : a.TotalVolume;
      const bVolume = typeof b.TotalVolume === 'string' ? parseFloat(b.TotalVolume.replace(/[\$,]/g, '')) : b.TotalVolume;
      return isAscending ? aVolume - bVolume : bVolume - aVolume;
    });
    setFilteredPools({ ...filteredPools, TableData: sortedTableData });
    setIsAscending(!isAscending);
  };

  // Sorting logic for APR
  const sortApr = () => {
    if (!filteredPools?.TableData) return;
    const sortedTableData = [...filteredPools.TableData].sort((a, b) => {
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
    setFilteredPools({ ...filteredPools, TableData: sortedTableData });
    setIsAscending(!isAscending);
  };

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

  const handleShowMore = () => {
    setDisplayCount(prevCount => prevCount + 10);
  };

  return (
    <div className='max-w-[1200px] mx-auto h-screen relative'>
      {/* search box */}
      <div className="flex justify-end items-center">
        <div className="relative w-full max-w-lg">
          <div className="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
            <svg className="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M21 21l-4.35-4.35M17 10.5A6.5 6.5 0 104 10.5a6.5 6.5 0 0013 0z"></path>
            </svg>
          </div>

          <input
            type="text"
            placeholder="Search..."
            className="w-full py-3 pl-10 pr-4 bg-transparent rounded-lg shadow-inner  text-gray-400 placeholder-gray-400 border border-transparent border-blue-300 hover:border-blue-400 focus:ring-2 transition duration-200 ease-in-out"
            value={filterData}
            onChange={(e) => setFilterData(e.target.value)}
          />
        </div>
      </div>

      <div className='w-full h-screen text-white mt-4 z-20 mx-auto absolute'>
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
          <div className='-my-2 overflow-x-auto scroll-smooth'>
            <div className='inline-block min-w-full py-2 align-middle'>
              <div className='overflow-hidden shadow ring-1 ring-black ring-opacity-5'>
                <SkeletonTheme baseColor="#1f2029" highlightColor="#2b2b2b" borderRadius="0.5rem" duration={2}>
                  <table className='min-w-[1000px] md:min-w-[1200px]'>
                    <thead>
                      <tr>
                        {['Pool Name', 'Value', 'Total Volume', 'APR'].map((heading, index) => (
                          <th
                            scope='col'
                            key={index}
                            className='pl-10 pr-3 text-sm font-medium text-center text-white py-7 md:text-base lg:text-xl'
                          >
                            <span className='flex cursor-pointer items-center gap-2' onClick={() => sortingConditional(index)}>
                              {heading}
                              {index === activeSort ? (
                                isAscending ? (
                                  <ArrowUpwardIcon sx={{ color: '' }} />
                                ) : (
                                  <ArrowDownwardIcon sx={{ color: '' }} />
                                )
                              ) : index !== 0 ? <ImportExportIcon /> : ""}
                            </span>
                          </th>
                        ))}
                      </tr>
                    </thead>
                    <tbody>
                      {!filteredPools
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
                        : filteredPools?.slice(0, displayCount).map((pool, index) => (
                          <tr key={index} className='min-w-[1000px] '>
                            <td className='flex items-center  pl-10 pr-3 gap-2 md:gap-5 my-4 text-sm font-medium text-white min-w-52 whitespace-nowrap md:text-base'>
                              <span className='flex items-center gap-x-2 flex-wrap gap-y-2'>
                                {pool[1][0]?.pool_data?.map((token) => (
                                  <div className='flex items-center gap-x-1 border-2 rounded-2xl py-1 px-2 ' key={token.token_name}>
                                    <img className='w-6 h-6' src={token.image} alt="" />
                                    <span>{token.token_name}</span>
                                    <span>{token.weight * 100}%</span>
                                  </div>
                                ))}
                              </span>
                            </td>
                            <td className='px-3 py-4 text-sm pl-10 pr-3 text-white whitespace-nowrap md:text-base'>
                              $ {pool.PoolValue?.toLocaleString('en-US') || "46466464"}
                            </td>
                            <td className='px-3 py-4 text-sm pl-12 pr-3 text-white whitespace-nowrap md:text-base'>
                              $ {pool.TotalVolume?.toLocaleString('en-US') || "35355"}
                            </td>
                            <td className='py-4  text-sm font-medium pl-10 pr-3 whitespace-nowrap md:text-base'>
                              {pool.APR || "04% - 6%"}
                            </td>
                          </tr>
                        ))}
                    </tbody>
                  </table>
                </SkeletonTheme>
                {/* Show More button */}
                {filteredPools && displayCount < filteredPools.length && (
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
