import React, { useState } from 'react';
import "./SearchBar.css"

const SearchBar = () => {
    const [value, setValue] = useState({});

    const handleInputChange = (e) => setValue({
        ...value,
        [e.currentTarget.name]: e.currentTarget.value
    })

    return (
        <div className="searchBar">
            <form>
                <input type="text" name="search artist" onChange={handleInputChange} />
            </form>
        </div>
    );
};

export default SearchBar;