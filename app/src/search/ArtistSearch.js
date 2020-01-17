import React, { useState } from 'react';
import "./ArtistSearch.css";
import SearchBar from './SearchBar';
import ArtistResult from './ArtistResult'

const ArtistSearch = () => {
    return (
        <div className="artistSearch">
            <SearchBar />            
            <ArtistResult 
                name='White Buffalo' 
                src='https://i.scdn.co/image/54dd1789626208d0921f116cb15584e1f34a5ab7' />
            <ArtistResult
                name='Pavement'
                src='https://i.scdn.co/image/80c16daae85fd73f63aa9d64a30aa4029056bea3' />
            <ArtistResult
                name='Led Zeppelin'
                src='https://i.scdn.co/image/207803ce008388d3427a685254f9de6a8f61dc2e' />
        </div>
    )
};

export default ArtistSearch;