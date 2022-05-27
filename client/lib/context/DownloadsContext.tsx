import { Download } from "../graphql-operations";
import { createContext, Dispatch, FC, ReactNode, useReducer } from "react";
import { DownloadActions, downloadReducer } from "../reducers/download";

export interface DownloadState {
  downloads: Download[];
}

const initialState: DownloadState = { downloads: [] };

const mainReducer = (
  { downloads }: DownloadState,
  action: DownloadActions
) => ({
  downloads: downloadReducer(downloads, action),
});

const DownloadsContext = createContext<{
  state: DownloadState;
  dispatch: Dispatch<DownloadActions>;
}>({
  state: initialState,
  dispatch: () => null,
});

type DownloadsProviderProps = { children: ReactNode };
const DownloadsProvider: FC<DownloadsProviderProps> = ({ children }) => {
  const [state, dispatch] = useReducer(mainReducer, initialState);

  return (
    <DownloadsContext.Provider value={{ state, dispatch }}>
      {children}
    </DownloadsContext.Provider>
  );
};

export { DownloadsContext, DownloadsProvider };
