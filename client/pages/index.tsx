import type { NextPage } from "next";
import Head from "next/head";
import Image from "next/image";
import styles from "../styles/Home.module.css";
import { FC, useEffect, useState } from "react";
import axios from "axios";

type GetDownloadResponse = {
  id: string;
  link: string;
  state: string;
  file_name?: string;
};

interface DownloadingFileProps {
  id: string;
}

const DownloadingFile: FC<DownloadingFileProps> = (props) => {
  const [fileUrl, setFileUrl] = useState<GetDownloadResponse | undefined>();

  useEffect(() => {
    axios
      .get(`${process.env.NEXT_PUBLIC_BACKEND_URL}/download/${props.id}`)
      .then((res) => {
        setFileUrl(res.data);
      })
      .catch(console.error);
  }, [props.id]);

  if (!fileUrl) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <a
        href={`${process.env.NEXT_PUBLIC_BACKEND_URL}/download/${props.id}/file`}
        download={fileUrl.file_name}
      >
        {fileUrl.file_name}
      </a>
    </div>
  );
};

const Home: NextPage = () => {
  const [downloadingFiles, setDownloadingFiles] = useState<string[]>([]);
  const [url, setUrl] = useState("");

  const initiateDownload = (downloadLink: string) => {
    axios
      .post(
        `${process.env.NEXT_PUBLIC_BACKEND_URL}/download`,
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
      <main className={styles.main}>
        <h1 className={styles.title}>Download file</h1>
        <form
          onSubmit={(e) => {
            e.preventDefault();

            if (url !== "") {
              initiateDownload(url);
            }
          }}
        >
          <input
            name="url"
            type="text"
            placeholder="File to download"
            required
            value={url}
            onChange={(e) => setUrl(e.target.value)}
          />
          <button type="submit">Download</button>
        </form>

        <section>
          <h2>Downloaded files</h2>

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
      </main>
    </div>
  );
};

export default Home;
