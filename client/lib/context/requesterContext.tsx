import { createContext, FC, ReactNode, useEffect, useState } from "react";
import { v4 as uuidv4 } from "uuid";

export interface IRequesterContext {
  requesterId?: string;
  setRequesterId: (requesterId: string) => void;
}

const defaultState: IRequesterContext = {
  requesterId: undefined,
  setRequesterId: (requesterId: string) => {},
};

const RequesterContext = createContext<IRequesterContext>(defaultState);

interface RequesterProviderProps {
  children: ReactNode;
}

export const RequesterProvider: FC<RequesterProviderProps> = ({ children }) => {
  const [requesterId, setRequesterId] = useState<string | undefined>();

  useEffect(() => {
    // see if the user already has downloaded some files before
    const rawDownloads = localStorage.getItem("requester_id");
    if (rawDownloads) {
      setRequesterId(rawDownloads);
    } else {
      const uuid = uuidv4();
      localStorage.setItem("requester_id", uuid);
      setRequesterId(uuid);
    }
  }, []);

  return (
    <RequesterContext.Provider
      value={{
        requesterId,
        setRequesterId,
      }}
    >
      {children}
    </RequesterContext.Provider>
  );
};

export default RequesterContext;
