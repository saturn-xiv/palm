import { useState } from "react";
// import type { RpcError } from "grpc-web";

// import { PodmanClient } from "./protocols/MonitoringServiceClientPb";
// import {
//   PodmanQueryRequest,
//   PodmanLogsResponse,
// } from "./protocols/monitoring_pb";

import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";

function App() {
  const [count, setCount] = useState(0);
  // const client = new PodmanClient("http://localhost:8080", null, null);
  // const request = new PodmanQueryRequest();
  // client.logs(
  //   request,
  //   { "custom-header-1": "value1" },
  //   (err: RpcError, response: PodmanLogsResponse) => {
  //     if (err) {
  //       console.log(`${err.code} ${err.message}`);
  //       return;
  //     }
  //     console.log(response.getPagination()?.getTotal());
  //   }
  // );

  return (
    <>
      <div>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  );
}

export default App;
