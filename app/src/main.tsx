import React from "react";
import ReactDOM from "react-dom/client";
import AppContextProvider from "./context/AppContext";
import Dashboard from "./pages/Dashboard";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AppContextProvider>
      <Dashboard />
    </AppContextProvider>
  </React.StrictMode>,
);
