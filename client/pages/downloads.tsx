import {NextPage} from "next";
import {gql, useQuery, useSubscription} from '@apollo/client'

const GET_DOWNLOAD = gql`
    query GetDownload($downloadId: ID!) {
        getDownload(downloadId: $downloadId) {
            id
            link
            percentage
        }
    }
`

const SUBSCRIBE_DOWNLOAD = gql`
    subscription SubscribeDownload($downloadId: ID!) {
        getDownload(downloadId: $downloadId) {
            download {
                id
                link
                file
                percentage
            }
        }
    }
`

const DownloadsPage: NextPage = () => {
    const subscription = useSubscription(SUBSCRIBE_DOWNLOAD, {variables: {downloadId: "ca248725-75b6-43c1-b1e5-d1fe7a97e723"}})

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