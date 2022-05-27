import { FC } from "react";
import { useSubscription } from "@apollo/client";
import { SubscribeDownloadDocument } from "../../lib/graphql-operations";
import styles from "../../styles/index.module.scss";

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
      {download.state === "initiated" && download.percentage !== 100 && (
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

export default DownloadingFile;
