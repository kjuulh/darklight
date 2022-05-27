import { useContext } from "react";
import RequesterContext, {
  IRequesterContext,
} from "../context/requesterContext";

export const useRequesterContext = () => {
  return useContext(RequesterContext);
};

export const useRequesterQuery = <T,>(
  queryFunc: (ctx: IRequesterContext) => T
): T => {
  const ctx = useRequesterContext();

  return queryFunc(ctx);
};
