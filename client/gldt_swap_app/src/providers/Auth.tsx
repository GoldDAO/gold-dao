import { ReactNode } from "react";
import { Provider } from "@amerej/artemis-react";

const AuthProvider = ({ children }: { children: ReactNode }) => {
  return <Provider>{children}</Provider>;
};

export default AuthProvider;
