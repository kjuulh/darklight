import { FC, useContext, useState } from "react";
import styles from "../../styles/index.module.scss";
import { useMutation } from "@apollo/client";
import { RequestDownloadDocument } from "../../lib/graphql-operations";
import { useRequesterQuery } from "../../lib/hooks/requester";
import { DownloadsContext } from "../../lib/context/DownloadsContext";
import { Types } from "../../lib/reducers/base";

const AddDownload: FC = (props) => {
  const [url, setUrl] = useState<string>("");
  const { dispatch } = useContext(DownloadsContext);
  const [requesterId, loading] = useRequesterQuery((ctx) => [
    ctx.requesterId,
    !ctx.requesterId,
  ]);
  const [requestDownload, { data, error }] = useMutation(
    RequestDownloadDocument
  );

  const initiateDownload = (downloadLink: string) => {
    if (loading || !requesterId) {
      return;
    }

    requestDownload({
      variables: {
        link: downloadLink,
        requesterId,
      },
    })
      .then((res) => res.data?.requestDownload)
      .then((data) => {
        if (data) {
          dispatch({
            type: Types.Request,
            payload: { id: data.id, requesterId },
          });
        }
      })
      .catch(console.error);
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
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
  );
};

export default AddDownload;
