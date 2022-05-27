import { FC, ReactNode, useContext, useEffect, useState } from "react";
import styles from "./DefaultLayout.module.scss";
import { v4 as uuidv4 } from "uuid";
import RequesterContext from "../../lib/context/requesterContext";

interface DefaultLayoutProps {
  children: ReactNode;
}

const DefaultLayout: FC<DefaultLayoutProps> = ({ children }) => {
  const { requesterId, setRequesterId } = useContext(RequesterContext);

  if (requesterId === null) {
    return <div>Loading...</div>;
  }

  return (
    <div>
      <div className={styles.container}>
        <nav className={styles.nav}>
          <div className={styles.nav_container}>
            <div className={styles.icon}></div>
            <p>Darklight</p>
          </div>
        </nav>
        <main className={styles.main}>{children}</main>
      </div>
    </div>
  );
};

export default DefaultLayout;
