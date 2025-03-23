import { Outlet, useLocation, useNavigate } from "react-router-dom";

import { useAuth } from "@auth/index";

import { GLDT_STAKE_CANISTER_ID } from "@constants";

import { Button } from "@components/index";
import { useGetPoolStats } from "@services/gldt_stake/useGetPoolStats";
import { divideBy1e8 } from "@utils/numbers";

const NotConnected = () => {
  return (
    <div className="flex justify-center pt-20">
      <p>Login to start staking</p>
    </div>
  );
};

export const Stake = () => {
  const navigate = useNavigate();
  const { authenticatedAgent, isConnected, principalId } = useAuth();
  const navUrls = {
    create: `/stake/create`,
    my_positions: `/stake/my-positions`,
  };
  const navigateToCreateStake = () => {
    navigate(navUrls.create);
  };
  const navigateToMyPositions = () => {
    navigate(navUrls.my_positions);
  };
  const location = useLocation();
  const activeMenuItem = location.pathname;

  const { total_staked_query } = useGetPoolStats(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    { owner: principalId, enabled: !!isConnected }
  );
  if (total_staked_query.isLoading) return <p>Loading...</p>;
  if (total_staked_query.isError)
    return <p>Error: {total_staked_query.error?.message}</p>;

  const total_staked = Number(total_staked_query.data) || 0;
  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Stake</div>
      </div>
      {isConnected && principalId ? (
        <section className="rounded-xl mb-6 p-6">
          <div className="flex items-center justify-center px-6 pt-6 mb-4 lg:mb-6 flex-col">
            <p>Total staked in Pool : {divideBy1e8(total_staked)} GLDT</p>
            <p>GLDT Current APY: 21%</p>
          </div>
          <div className="navbar flex justify-center gap-2 mb-4">
            <Button
              onClick={navigateToCreateStake}
              className={activeMenuItem === navUrls.create ? "bg-accent" : ""}
            >
              Stake GLDT
            </Button>
            <Button
              onClick={navigateToMyPositions}
              className={
                activeMenuItem === navUrls.my_positions ? "bg-accent" : ""
              }
            >
              My Stake positions
            </Button>
          </div>
          <Outlet />
        </section>
      ) : (
        <NotConnected></NotConnected>
      )}
    </>
  );
};
