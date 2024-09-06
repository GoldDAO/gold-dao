// import { useEffect } from "react";

import { useTransfer } from "@context/index";

const Transfer = () => {
  const { state: transferState } = useTransfer();
  const { token } = transferState;

  // const handleOnChangeToken = (token: Token): void => {
  //   setToken(token);
  // };

  return <div>TRANSFER {token}</div>;
};

export default Transfer;
