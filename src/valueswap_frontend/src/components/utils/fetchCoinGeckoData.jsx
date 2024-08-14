import React from 'react'

export const fetchCoinGeckoData = async() => {
  const url = 'https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=chain-key-bitcoin&category=internet-computer-ecosystem';
  const options = {
    method: 'GET',
    headers: {
      accept: 'application/json',
      'x-cg-demo-api-key': '	CG-7sJQPqdKkGEMxvTvSCSfBnSA'
    }
  };

//    useEffect(()=>{
//     fetch(url, options)
//     .then(res => res.json())
//     .then(json => console.log(json))
//     .catch(err => console.error('error:' + err));
//    },[])
//     return;
try {
  const response = await fetch(url, options)
  
  if(!response.ok){
    throw new Error ("can't fetch data from api to show coin")
  }
  const data = await response.json()
  console.log(data)
  return data;
} catch (error) {
    console.error('Error fetching data:', error);
    throw error;
}
}


export const searchCoinGeckoData = async(search) => {
  const url = `https://api.coingecko.com/api/v3/search?query=${search}`;
  const options = {
    method: 'GET',
    headers: {
      accept: 'application/json',
      'x-cg-demo-api-key': '	CG-7sJQPqdKkGEMxvTvSCSfBnSA'
    }
  };

try {
  const response = await fetch(url, options)
  // console.log(response)
  if(!response.ok){
    throw new Error ("can't fetch data from api to show coin")
  }
  const data = await response.json()
  console.log(data)
  return data;
} catch (error) {
    console.error('Error fetching data:', error);
    throw error;
}
}


export const searchCoinGeckoById = async(search) => {
  const url = `https://api.coingecko.com/api/v3/coins/${search}`;
  const options = {
    method: 'GET',
    headers: {
      accept: 'application/json',
      'x-cg-demo-api-key': '	CG-7sJQPqdKkGEMxvTvSCSfBnSA'
    }
  };

try {
  const response = await fetch(url, options)
  // console.log(response)
  if(!response.ok){
    throw new Error ("can't fetch data from api to show coin")
  }
  const data = await response.json()
  console.log(data)
  return data;
} catch (error) {
    console.error('Error fetching data:', error);
    throw error;
}
}

