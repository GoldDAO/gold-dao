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
  const active = location.pathname;
  const { data, isSuccess } = useGetFullAccount({
    owner,
    subaccount,
  });

  const handleGoBack = () => {
    // const windowHS = window.history.state;

    // if (windowHS && windowHS.idx > 0) navigate(-1);
    // else navigate("/explorer");
    navigate("/explorer");
  };

  return (
    <div className={className}>
      <div className="flex items-center gap-2">
        <div
          onClick={handleGoBack}
          className="hover:text-accent cursor-pointer"
        >
          Explorer
        </div>
        <div>/</div>
        {active.includes("account") && isSuccess && (
          <div className="max-w-48 lg:max-w-96 cursor-default">
            <div className="truncate">{data}</div>
          </div>
        )}
      </div>
    </div>
  );
};
