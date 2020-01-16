import React, { useState } from 'react';
import './App.css';
import './search/ArtistSearch'
import ArtistSearch from './search/ArtistSearch';

const Loaded = ({ wasm }) => <button onClick={wasm.greet}>Click me</button>;

const Unloaded = ({ loading, loadWasm }) => {
  return loading ? (
    <div>Loading...</div>
  ) : (
    <button onClick={loadWasm}>Load library</button>
  );
};

const App = () => {
  const [loading, setLoading] = useState(false);
  const [wasm, setWasm] = useState(null);

  const loadWasm = async () => {
    try {
      setLoading(true);
      const wasm = await import('sonic-roadtrip');
      setWasm(wasm);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="App">
      <ArtistSearch />
    </div>
  );
};

export default App;
