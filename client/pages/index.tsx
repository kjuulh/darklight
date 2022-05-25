import type { NextPage } from "next";
import { FC, useEffect, useState } from "react";

import styles from "../styles/index.module.scss";
import { useMutation, useSubscription } from "@apollo/client";
import {
  RequestDownloadDocument,
  SubscribeDownloadDocument,
} from "../lib/graphql-operations";

type GetDownloadResponse = {
  id: string;
  link: string;
  state: string;
  file_name?: string;
  percentage: number;
};

interface DownloadingFileProps {
  id: string;
}

const DownloadingFile: FC<DownloadingFileProps> = (props) => {
  const { data, loading } = useSubscription(SubscribeDownloadDocument, {
    variables: { downloadId: props.id },
  });

  if (loading) {
    return <div>Loading...</div>;
  }

  if (!data?.getDownload?.download) {
    return <div>File was not found</div>;
  }

  const { download } = data.getDownload;

  return (
    <div>
      <a
        href={`${process.env.NEXT_PUBLIC_BACKEND_URI}api/download/${props.id}/file`}
        download={download.file}
      >
        {download.file}
      </a>
      {download.state === "initiated" && (
        <>
          <div>
            <div className={styles.progress_bar}>
              <div
                className={styles.progress_bar__complete}
                style={{ width: `${download.percentage}%` }}
              ></div>
            </div>
          </div>
        </>
      )}
    </div>
  );
};

const Home: NextPage = () => {
  const [downloadingFiles, setDownloadingFiles] = useState<string[]>([]);
  const [url, setUrl] = useState("");

  const [requestDownload, { data, loading, error }] = useMutation(
    RequestDownloadDocument
  );

  useEffect(() => {
    // see if the user already has downloaded some files before
    const rawDownloads = localStorage.getItem("downloads");
    if (rawDownloads) {
      setDownloadingFiles((df) => {
        return JSON.parse(rawDownloads) as string[];
      });
    }
  }, []);

  const initiateDownload = (downloadLink: string) => {
    requestDownload({
      variables: {
        link: downloadLink,
        requesterId: "e3ff46f2-d4ae-4fa2-abc1-8960a23a34f3",
      },
    })
      .then((res) => {
        console.log("downloaded" + downloadLink);
        return res.data?.requestDownload;
      })
      .then((data) => {
        if (data) {
          setDownloadingFiles((files) => [...files, data.id]);
        }
      })
      .catch(console.error);
  };

  return (
    <div className={styles.container}>
      <nav className={styles.nav}>
        <div className={styles.nav_container}>
          <div className={styles.icon}></div>
          <p>Darklight</p>
        </div>
      </nav>
      <main className={styles.main}>
        <h1 className={styles.title}>Download file</h1>

        <form
          className={styles.form}
          onSubmit={(e) => {
            e.preventDefault();

            if (url !== "") {
              initiateDownload(url);
            }
          }}
        >
          <div className={styles.input_group}>
            <input
              className={styles.download_input}
              name="url"
              type="text"
              placeholder="File to download"
              required
              value={url}
              autoComplete="off"
              onChange={(e) => setUrl(e.target.value)}
            />
            <button type="submit">Download</button>
          </div>
        </form>

        {downloadingFiles.length > 0 && (
          <section>
            <h2 className={styles.sub_title}>Downloaded files</h2>

            <div>
              <ul>
                {downloadingFiles.map((df, i) => (
                  <div key={i}>
                    <DownloadingFile id={df} />
                  </div>
                ))}
              </ul>
            </div>
          </section>
        )}
      </main>
    </div>
  );
};

export default Home;
