import reactLogo from './assets/react.svg'
import './App.css'
import { useEffect, useState } from 'react';
import { initialize } from './domain';
import { CounterState } from '../@rsw/my-wasm-test/pkg/my_wasm_test';

function App(){
  const [counter, setCounter] = useState<CounterState | undefined>();
  const [count, setCount] = useState<number>(0);

  useEffect(() => {
    initialize().then(counter => {
      setCounter(counter);
      setCount(counter.get_counter());
    });
  }, []);

  function handleClick() {
    console.log('clicked!');
    const updatedCount = counter?.increment_counter();
    if (updatedCount) {
      setCount(updatedCount);
    }
  }

  return (
    <div className="App">
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo" alt="Vite logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>

      <div className="card">
        <button onClick={handleClick}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  )
}

export default App
