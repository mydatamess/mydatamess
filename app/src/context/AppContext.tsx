import React, { createContext } from "react";
import ResourceService from "../services/resources/ResourceService";
import { TauriResourceService } from "../services/resources/TauriResourceService";

export type AppContextType = {
  services: {
    resourceService: ResourceService;
  };
};

const AppContext = createContext<AppContextType>({} as AppContextType);

const AppContextProvider = ({ children }: { children: React.ReactNode }) => {
  const resourceService = new TauriResourceService();

  return (
    <AppContext.Provider
      value={{
        services: {
          resourceService,
        },
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export const useAppContext = () => {
  return React.useContext(AppContext);
};

export default AppContextProvider;
