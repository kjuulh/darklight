import type { NextPage } from "next";
import { FC, useEffect, useState } from "react";
import axios from "axios";

import styles from "../styles/index.module.scss";

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
  const [fileUrl, setFileUrl] = useState<GetDownloadResponse | undefined>();

  const fetchFileUrlUpdate = (downloadId: string) => {
    axios
      .get<GetDownloadResponse>(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/download/${props.id}`
      )
      .then((res) => {
        setFileUrl(res.data);
        return res.data;
      })
      .then((res) => {
        if (res.state === "initiated") {
          setTimeout(() => {
            fetchFileUrlUpdate(res.id);
          }, 2000);
        }
      })
      .catch(console.error);
  };
  useEffect(() => {
    axios
      .get<GetDownloadResponse>(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/download/${props.id}`
      )
      .then((res) => {
        setFileUrl(res.data);
        const rawDownloads = localStorage.getItem("downloads");
        if (rawDownloads) {
          const downloads = JSON.parse(rawDownloads) as string[];
          const newDownloads = [
            ...downloads.filter((d) => d !== res.data.id),
            res.data.id,
          ];
          localStorage.setItem("downloads", JSON.stringify(newDownloads));
        } else {
          localStorage.setItem("downloads", JSON.stringify([res.data.id]));
        }

        return res.data;
      })
      .then((res) => {
        if (res.state === "initiated") {
          fetchFileUrlUpdate(res.id);
        }
      })
      .catch(console.error);
  }, [props.id]);

  if (!fileUrl) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <a
        href={`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/download/${props.id}/file`}
        download={fileUrl.file_name}
      >
        {fileUrl.file_name}
      </a>
      {fileUrl.state === "initiated" && (
        <div>
          <div className={styles.progress_bar}>
            <div
              className={styles.progress_bar__complete}
              style={{ width: `${fileUrl.percentage}%` }}
            ></div>
          </div>
        </div>
      )}
    </div>
  );
};

const Home: NextPage = () => {
  const [downloadingFiles, setDownloadingFiles] = useState<string[]>([]);
  const [url, setUrl] = useState("");

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
    axios
      .post(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/api/download`,
        {
          link: downloadLink,
        },
        {}
      )
      .then((res) => {
        setDownloadingFiles((files) => [...files, res.data]);
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
                  <li key={i}>
                    <DownloadingFile id={df} />
                  </li>
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
