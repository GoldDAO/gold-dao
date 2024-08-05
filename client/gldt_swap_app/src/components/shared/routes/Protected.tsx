import { Navigate, Outlet } from "react-router-dom";
// import { useWallet } from "@amerej/artemis-react";
// import { Dialog, LoaderSpin } from "@components/ui";

const ProtectedRoute = () => {
  // const { isConnected, state, walletState } = useWallet();
  const walletId = localStorage.getItem("dfinityWallet") || "";

  if (!walletId) {
    return <Navigate to="/swap" replace />;
  }

  return <Outlet />;
};

export default ProtectedRoute;
