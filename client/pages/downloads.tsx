import {NextPage} from "next";
import {gql, useQuery, useSubscription} from '@apollo/client'
import {SubscribeDownloadDocument} from "../lib/graphql-operations";

const DownloadsPage: NextPage = () => {
    const subscription = useSubscription(SubscribeDownloadDocument, {variables: {downloadId: "ca248725-75b6-43c1-b1e5-d1fe7a97e723"}})

    if (subscription.loading) {
        return <div>Loading...</div>
    }

    return <div>
        <div>
            {JSON.stringify(subscription.data)}
        </div>
    </div>
}

export default DownloadsPage