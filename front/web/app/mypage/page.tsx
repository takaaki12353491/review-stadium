import { Authenticator } from "@aws-amplify/ui-react";
import "../styles/globals.css";
import "antd/dist/antd.dark.css";
import "@aws-amplify/ui-react/styles.css";

export default function Page() {
  return (
    <Authenticator.Provider>
      <div>mypage</div>
    </Authenticator.Provider>
  );
}
