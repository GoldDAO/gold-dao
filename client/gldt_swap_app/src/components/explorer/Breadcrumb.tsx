import { useLocation, useNavigate } from "react-router-dom";
import { useGetFullAccount } from "@hooks/useGetFullAccount";

export const Breadcrumb = ({
  className = "",
  owner,
  subaccount,
}: {
  className?: string;
  owner: string;
  subaccount: string | undefined;
}) => {
  const navigate = useNavigate();
  const location = useLocation();
  const previousExplorerTab = location.pathname.split("/")[2];
  const active = location.pathname;
  const { data, isSuccess } = useGetFullAccount({
    owner,
    subaccount,
  });

  const handleNavigateExplorer = () => {
    navigate("/explorer");
  };

  const handleNavigatePreviousExplorerTab = () => {
    const windowHS = window.history.state;
    if (windowHS && windowHS.idx > 0) navigate(-1);
    else navigate("/explorer");
  };

  const renderPreviousExplorerTab = () => {
    if (previousExplorerTab === "top_holders") return "Top Holders";
    else if (previousExplorerTab === "transactions") return "Transactions";
  };

  return (
    <div className={className}>
      <div className="flex items-center gap-2">
        <div
          onClick={handleNavigateExplorer}
          className="hover:text-accent cursor-pointer"
        >
          Explorer
        </div>
        <div>/</div>
        {previousExplorerTab === "top_holders" && (
          <>
            <div
              className="hover:text-accent cursor-pointer"
              onClick={handleNavigatePreviousExplorerTab}
            >
              {renderPreviousExplorerTab()}
            </div>
            <div>/</div>
          </>
        )}

        {active.includes("account") && isSuccess && (
          <div className="max-w-48 lg:max-w-96 cursor-default">
            <div className="truncate">{data}</div>
          </div>
        )}
      </div>
    </div>
  );
};
