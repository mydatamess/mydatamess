import React, { useState } from "react";
import reactLogo from "./assets/react.svg";
import "./App.css";
import { useAppContext } from "./context/AppContext";

function App(): React.JSX.Element {
  const {
    services: { resourceService },
  } = useAppContext();
  const [resources, setResources] = useState<string[]>([]);

  async function greet() {
    setResources(await resourceService.getResources());
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank" rel="noreferrer">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank" rel="noreferrer">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <ul>
        {resources.map((resource) => (
          <li key={resource}>{resource}</li>
        ))}
      </ul>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input id="greet-input" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>
    </main>
  );
}

export default App;
