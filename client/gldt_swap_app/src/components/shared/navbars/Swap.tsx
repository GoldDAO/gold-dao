import { Link, useNavigate } from "react-router-dom";
import { useWallet } from "@amerej/artemis-react";
import { UserIcon } from "@heroicons/react/20/solid";
import Auth from "@components/shared/Auth";
import { Tile, Skeleton } from "@components/ui";

const Swap = () => {
  const { isConnected, principalId } = useWallet();
  const navigate = useNavigate();

  const handleNavigateAccountPage = () => {
    navigate("/swap/account");
  };

  return (
    <>
      <nav className="sticky top-0 px-6 py-5 z-40 bg-background">
        <div className="grid grid-cols-2 items-center h-10">
          <div className="flex-shrink-0">
            <Link to="/swap" className="flex items-center space-x-2">
              <img src="/vite.svg" alt="GLDT Logo" />
              <span className="self-center text-xl font-semibold whitespace-nowrap hidden sm:block">
                GLDT Swap App
              </span>
            </Link>
          </div>
          <div className="flex justify-self-end items-center">
            {!isConnected && <Auth />}
            {isConnected && (
              <button className="flex items-center bg-surface rounded-full py-1 px-1">
                <div
                  className="flex items-center"
                  onClick={handleNavigateAccountPage}
                >
                  <Tile className="rounded-full h-8 w-8 bg-background">
                    <UserIcon className="p-1 text-white" />
                  </Tile>
                  <div className="hidden sm:block">
                    <div className="flex items-center truncate pr-4">
                      <div className="flex ml-4 items-center truncate text-sm max-w-64">
                        <div className="mr-2 shrink-0">Principal ID: </div>
                        {principalId ? (
                          <>
                            <div
                              className="truncate"
                              data-tooltip-id="tooltip_principal_id"
                              data-tooltip-content={principalId}
                            >
                              {principalId}
                            </div>
                          </>
                        ) : (
                          <Skeleton className="w-64" />
                        )}
                      </div>
                    </div>
                  </div>
                </div>
              </button>
            )}
          </div>
        </div>
      </nav>
    </>
  );
};

export default Swap;
