import { Navigate, Outlet } from "react-router-dom";

const ProtectedRoute = () => {
  const connected = localStorage.getItem("connected") || "";

  if (connected && connected === "1") return <Outlet />;
  return <Navigate to="/swap" replace />;
};

export default ProtectedRoute;
