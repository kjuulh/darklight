import { useContext, useReducer } from "react";
import { DownloadsContext, DownloadState } from "../context/DownloadsContext";

export const useSelector = <T,>(selectorFunc: (state: DownloadState) => T) => {
  const ctx = useContext(DownloadsContext);
  return selectorFunc(ctx.state);
};

export const useDownloadContext = () => useContext(DownloadsContext);
export const useDownloadDispatch = () => {
  const { dispatch } = useDownloadContext();
  return dispatch;
};
