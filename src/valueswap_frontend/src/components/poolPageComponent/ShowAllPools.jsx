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
import BorderGradientButton from '../../buttons/BorderGradientButton';
import KeyboardArrowRightIcon from '@mui/icons-material/KeyboardArrowRight';
import KeyboardArrowLeftIcon from '@mui/icons-material/KeyboardArrowLeft';
import KeyboardDoubleArrowLeftIcon from '@mui/icons-material/KeyboardDoubleArrowLeft';
import KeyboardDoubleArrowRightIcon from '@mui/icons-material/KeyboardDoubleArrowRight';
const ShowAllPools = () => {
  const [allDataInPool, setAllDataInPool] = useState(null);
  const [filteredPools, setFilteredPools] = useState([]); // To store filtered results
  const [isAscending, setIsAscending] = useState(true); // Tracks sorting order
  const [activeSort, setActiveSort] = useState(null);
  const [currentPage, setCurrentPage] = useState(1); // Tracks which column is being sorted
  const [itemsPerPage] = useState(10); // Tracks how many rows to display
  const { backendActor } = useAuth();
  const navigate = useNavigate();
  const [filterData, setFilterData] = useState(""); // Search input value
  const ref = useRef();


  const dummyPoolsData = [
    {
      PoolId: "1",
      PoolValue: "$123,456",
      TotalVolume: "$12,345",
      APR: "3.5% - 6.0%",
      pool_data: [
        {
          token_name: "TokenA",
          weight: 0.5,
          image: "https://via.placeholder.com/50",
        },
        {
          token_name: "TokenB",
          weight: 0.5,
          image: "https://via.placeholder.com/50",
        },
      ],
    },
    {
      PoolId: "2",
      PoolValue: "$234,567",
      TotalVolume: "$23,456",
      APR: "4.0% - 7.0%",
      pool_data: [
        {
          token_name: "TokenC",
          weight: 0.7,
          image: "https://via.placeholder.com/50",
        },
        {
          token_name: "TokenD",
          weight: 0.3,
          image: "https://via.placeholder.com/50",
        },
      ],
    },
    {
      PoolId: "3",
      PoolValue: "$345,678",
      TotalVolume: "$34,567",
      APR: "5.0% - 8.0%",
      pool_data: [
        {
          token_name: "TokenE",
          weight: 0.6,
          image: "https://via.placeholder.com/50",
        },
        {
          token_name: "TokenF",
          weight: 0.4,
          image: "https://via.placeholder.com/50",
        },
      ],
    },
  ];
  
  // Fetch pool data from backend
  // console.log("allPool a")
  useEffect(() => {
    const fetchPoolData = async () => {
      try {
        const AllPoolsData = await valueswap_backend?.get_pool_data();
        if (AllPoolsData.Ok[0].length > 0) {
    
         
          setAllDataInPool(AllPoolsData.Ok[0]); // Set the fetched data
          setFilteredPools(AllPoolsData.Ok[0]); // Initially set filteredPools to all data
        }
      
        return;
      } catch (error) {
        console.error("Error fetching pool data", error);
      }
    };
    fetchPoolData();
  }, []);

  // Filter logic based on search input
  useEffect(() => {
    if (filterData.trim() !== "") {
      const filtered = allDataInPool.Ok[0]?.filter(poolData =>
        poolData[1][0].pool_data.some(token =>
          token.token_name.toLowerCase().includes(filterData.toLowerCase())
        )
      );
      setFilteredPools(filtered);
    } else {
      setFilteredPools(allDataInPool);
    }
  }, [filterData, allDataInPool]);



   // Calculate total pages for pagination
   const totalPages = Math.ceil(filteredPools?.length / itemsPerPage);

   // Calculate current items based on current page
   const currentItems = filteredPools?.slice(
     (currentPage - 1) * itemsPerPage,
     currentPage * itemsPerPage
   );


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

 
  const handleNextPage = () => {
    if (currentPage < totalPages) setCurrentPage(currentPage + 1);
  };

  const handlePrevPage = () => {
    if (currentPage > 1) setCurrentPage(currentPage - 1);
  };

  const handleFirstPage = () => {
    setCurrentPage(1);
  };

  const handleLastPage = () => {
    setCurrentPage(totalPages);
  };

  return (
    <div className='max-w-[1200px] mx-auto  relative'>


      <div className='w-full  text-white mt-4 z-20 mx-auto '>
        

        <div className='flex flex-col font-gilroy bg-gray-700 bg-clip-padding backdrop-filter backdrop-blur-sm bg-opacity-10 border  border-[#FFFFFF66] rounded-2xl p-8 mb-8'>
          <div className='-my-2 overflow-x-auto scroll-smooth'>
            <div className='inline-block min-w-full py-2 align-middle'>
              <div className='overflow-hidden shadow ring-1 ring-black ring-opacity-5'>
                <SkeletonTheme baseColor="#1f2029" highlightColor="#2b2b2b" borderRadius="0.5rem" duration={2}>
                  <table className='min-w-full'>
                    <thead className='border-b-2 border-[#FFFFFF66] '>
                      <tr className='mx-auto'>
                        {['Pool Name', 'TVL', 'Volume', 'APR'].map((heading, index) => (
                          <th
                            scope='col'
                            key={index}
                            className={`text-sm font-medium cursor-pointer text-white py-7 md:text-base lg:text-xl 
                               ${index === 0 ? 'text-left w-7/12' : index === 3 ? 'text-right' : 'text-center'}`}
                          >
                            <span
                              className={`flex ${index === 0 ? 'justify-start' : index === 3 ? 'justify-end' : 'justify-center'
                                } items-center gap-2 cursor-pointer`}
                              onClick={() => sortingConditional(index)}
                            >
                              {heading}
                              {index === activeSort ? (
                                isAscending ? (
                                  <ArrowUpwardIcon sx={{ color: '' }} />
                                ) : (
                                  <ArrowDownwardIcon sx={{ color: '' }} />
                                )
                              ) : index !== 0 ? <ImportExportIcon /> : ''}
                            </span>
                          </th>
                        ))}
                      </tr>
                    </thead>

                    <tbody>
                      {!currentItems
                        ? Array.from({ length: 8 }).map((_, index) => (
                          <tr key={index}>
                            <td className='px-0 py-4 text-sm text-center text-white whitespace-nowrap md:text-base'>
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
                        : currentItems[1]?.map((pool, index) => (
                          <tr key={index} className='min-w-[1000px] ' onClick={() => navigate(`/valueswap/pool/addLiquidity/${pool[0]}`)}>
                            <td className='flex items-center   pr-3 gap-2 md:gap-5 my-4 text-sm font-medium text-white min-w-52 whitespace-nowrap md:text-base'>
                              <span className='flex items-center gap-x-2 flex-wrap gap-y-2'>
                                {      console.log("allPool a", pool, index)}
                                {pool?.pool_data?.map((token) => (
                                  <div className='flex items-center gap-x-1 cursor-pointer border-2 rounded-2xl py-1 px-2 ' key={token.token_name}>
                                    <img className='w-6 h-6' src={token.image} alt="" />
                                    <span>{token.token_name}</span>
                                    <span>{Number(token.weight)}%</span>
                                  </div>
                                ))}
                              </span>
                            </td>
                            <td className='px-3 py-4 text-sm pl-10 pr-3 text-white whitespace-nowrap md:text-base'>
                             ${
                                
                                (()=> {
                                  const totalVolume = 
                                  pool?.pool_data?.reduce((sum, volume) => 
                                     sum + Number(volume.value), 0
                                  )
                                  return totalVolume;
                                })()
                            }
                            </td>
                            <td className='px-3 py-4 text-sm pl-12 pr-3 text-white whitespace-nowrap md:text-base'>
                             3
                              
                            </td>
                            <td className='py-4  text-sm font-medium  pr-3 whitespace-nowrap md:text-base'>
                              {pool.APR || "04% - 6%"}
                            </td>
                          </tr>
                        ))}
                    </tbody>
                  </table>
                </SkeletonTheme>
                {/* Show More button */}
                {/* filteredPools && displayCount < filteredPools.length  */}
                {dummyPoolsData && (
                  <div className='mt-4 text-center'>
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
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ShowAllPools;
