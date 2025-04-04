import React, { createContext } from "react";
import ResourceService from "../application/services/resources/ResourceService";
import { TauriResourceService } from "../application/services/resources/TauriResourceService";

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
