import type { NextPage } from "next";
import { useEffect } from "react";
import styles from "../styles/index.module.scss";
import { AddDownload, ListDownloads } from "../components/downloads";
import { DefaultLayout } from "../components/layouts";
import { useLazyQuery } from "@apollo/client";
import { GetDownloadsDocument } from "../lib/graphql-operations";
import { useRequesterQuery } from "../lib/hooks/requester";
import { useDownloadDispatch } from "../lib/hooks/downloads";
import { Types } from "../lib/reducers/base";

const useStartupQuery = (): { loading: boolean } => {
  const [requesterId, loading] = useRequesterQuery((ctx) => [
    ctx.requesterId,
    !ctx.requesterId,
  ]);
  const dispatch = useDownloadDispatch();
  const [request] = useLazyQuery(GetDownloadsDocument);

  useEffect(() => {
    if (loading || !requesterId) {
      return;
    }
    request({
      variables: {
        requesterId,
      },
    })
      .then((data) => {
        if (data.data?.getDownloads && data.data.getDownloads.length > 0) {
          dispatch({
            type: Types.LoadDownloads,
            payload: { downloads: data.data.getDownloads },
          });
        }
      })
      .catch(console.error);
  }, [dispatch, requesterId, loading, request]);

  return { loading };
};

const Home: NextPage = () => {
  const { loading } = useStartupQuery();

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <>
      <h1 className={styles.title}>Download file</h1>
      <AddDownload />
      <ListDownloads />
    </>
  );
};

// @ts-ignore
Home.layout = DefaultLayout;

export default Home;
