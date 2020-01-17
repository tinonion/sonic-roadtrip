import React, { useState } from 'react';
import "./ArtistResult.css";

const ArtistResult = (props) => {
    return (
        <div className="artistResult">
           <img src={props.src} className="artistImage"></img> 
           {props.name}
        </div>
    );
};

export default ArtistResult;
