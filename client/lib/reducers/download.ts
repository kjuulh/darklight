import { ActionMap, Types } from "./base";
import { Download } from "../graphql-operations";

type DownloadPayload = {
  [Types.Request]: {
    id: string;
    requesterId: string;
  };
  [Types.LoadDownloads]: {
    downloads: Download[];
  };
};

export type DownloadMap = ActionMap<DownloadPayload>;

export type DownloadActions = DownloadMap[keyof DownloadMap];

export const downloadReducer = (state: Download[], action: DownloadActions) => {
  switch (action.type) {
    case Types.Request:
      return [
        ...state,
        {
          id: action.payload.id,
          requesterId: action.payload.requesterId,
        } as unknown as Download,
      ];
    case Types.LoadDownloads:
      return [...state, ...action.payload.downloads];
    default:
      return state;
  }
};
