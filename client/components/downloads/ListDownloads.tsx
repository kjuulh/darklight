import { FC, useContext } from "react";
import { DownloadingFile } from "./index";
import { DownloadsContext } from "../../lib/context/DownloadsContext";

interface ListDownloadsProps {}

const ListDownloads: FC<ListDownloadsProps> = ({}) => {
  const {
    state: { downloads },
  } = useContext(DownloadsContext);

  if (downloads.length === 0) {
    return <div></div>;
  }

  return (
    <section>
      <ul>
        {downloads.map((df, i) => (
          <div key={i}>
            <DownloadingFile id={df.id} />
          </div>
        ))}
      </ul>
    </section>
  );
};
export default ListDownloads;
