import { useLocation, useNavigate } from "react-router-dom";

export const Breadcrumb = ({
  className = "",
  owner,
}: {
  className?: string;
  owner: string;
}) => {
  const navigate = useNavigate();
  const location = useLocation();
  const active = location.pathname;

  const handleGoBack = () => {
    const windowHS = window.history.state;

    if (windowHS && windowHS.idx > 0) navigate(-1);
    else navigate("/explorer");
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
        {active.includes("account") && (
          <div className="max-w-48 lg:max-w-64 cursor-default">
            <div className="truncate">{owner}</div>
          </div>
        )}
      </div>
    </div>
  );
};
