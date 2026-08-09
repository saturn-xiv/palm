import { useState } from "react";

import "./App.css";

import Password from "./portal/Password";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <div>
        <Password />
      </div>
      <button
        type="button"
        className="counter"
        onClick={() => setCount((count) => count + 1)}
      >
        Count is {count}
      </button>
    </>
  );
}

export default App;
