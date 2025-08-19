import clsx from "clsx";
import Icon from "@shared/ui/icons";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";
import GradientCard from "@shared/ui/card/GradientCard";
import { TOKEN_GLDT } from "@shared/utils/tokens";
import IncreaseStake from "./components/increase-stake";
import DecreaseStake from "./components/decrease-stake";
import ClaimRewardsDisclaimer from "./components/claim-rewards-disclaimer";
import TokenHeaderPrice from "@shared/components/token-header-price";
import UserTotalStakedAmount from "./components/user-total-staked-amount";
import DissolveEventsList from "./components/dissolve-events-list";
import useFetchUserPosition from "@earn/hooks/useFetchUserPosition";
import Withdraw from "./components/withdraw";
import StakeAPY from "./components/stake-apy";
import TotalStakedAmount from "./components/total-staked-amount";
import DissolveEventsListDisconnected from "./components/dissolve-events-list-disconnected";

const Earn = () => {
  const { isConnected, authenticatedAgent, unauthenticatedAgent } = useAuth();

  const position = useFetchUserPosition(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    unauthenticatedAgent,
    {
      enabled: isConnected && !!authenticatedAgent && !!unauthenticatedAgent,
    }
  );

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="text-4xl xl:text-6xl flex flex-col justify-center items-center xl:items-start">
          <div className="font-semibold text-gold/90">Earn</div>
          <div className="font-light">with gold</div>
        </div>
        <div className="flex flex-col items-center xl:items-start text-content/60 my-3">
          <div className="text-center xl:text-left">
            Stake your GLDT to{" "}
            <span className="font-semibold">earn weekly rewards</span> in
            governance tokens, unlocking passive income from your gold holdings.
          </div>
          <div className="mt-4">
            <a
              href="https://docs.gold-dao.org/resources/gldt-staking/"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-1"
            >
              <div className="text-sm tracking-widest">LEARN MORE</div>
              <div className="mb-0.5">
                <Icon.ExternalLink width={16} />
              </div>
            </a>
          </div>
        </div>

        <div className="mt-4 xl:mt-6 flex flex-col gap-4 w-full">
          <StakeAPY />
          <TotalStakedAmount />
        </div>

        {!isConnected && (
          <BtnConnectWallet className="hidden xl:block mt-auto w-full" />
        )}
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div>
          <GradientCard
            className={clsx(
              "px-4 xl:px-8 pt-4 xl:pt-8 pb-24",
              "rounded-tr-[inherit]",
              "relative"
            )}
          >
            <TokenHeaderPrice
              className="hidden xl:block mb-8 xl:mb-12"
              token={TOKEN_GLDT}
            />

            <UserTotalStakedAmount position={position} />

            <div className="flex justify-center gap-2 absolute -bottom-9 left-1/2 -translate-x-1/2">
              <IncreaseStake position={position} />
              <DecreaseStake position={position} />
            </div>
          </GradientCard>

          <div className="flex flex-col gap-4 xl:gap-8 pt-16 px-4 pb-4 xl:px-8 xl:pb-8">
            <ClaimRewardsDisclaimer position={position} />
            <div>
              <div className="flex justify-between gap-4 items-center mt-8">
                <h2>Unlocking tokens</h2>
                <Withdraw position={position} />
              </div>
              <div className="mt-4">
                {isConnected ? (
                  <DissolveEventsList position={position} />
                ) : (
                  <DissolveEventsListDisconnected />
                )}
              </div>
            </div>
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Earn;
