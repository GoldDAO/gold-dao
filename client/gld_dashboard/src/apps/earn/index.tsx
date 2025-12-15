import clsx from "clsx";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";
import GradientCard from "@shared/ui/card/GradientCard";
import { TOKEN_GLDT } from "@shared/utils/tokens";
import ClaimRewardsDisclaimer from "./components/claim-rewards-disclaimer";
import TokenHeaderPrice from "@shared/components/token-header-price";
import UserTotalStakedAmount from "./components/user-total-staked-amount";
import useFetchUserPosition from "@earn/hooks/useFetchUserPosition";
import Withdraw from "./components/withdraw";
import { LoaderSpin } from "@components/loaders";
import DissolveEventsListDisconnected from "./components/dissolve-events-list-disconnected";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const styles = {
  container: clsx("border border-border bg-surface-primary rounded-xl"),
  title: clsx("text-content text-center xl:text-left"),
  totalAmount: clsx("font-semibold text-xl"),
  description: clsx("text-sm text-content/60"),
};

const Earn = () => {
  const { isConnected, unauthenticatedAgent, principalId } = useAuth();

  const position = useFetchUserPosition(GLDT_STAKE_CANISTER_ID, {
    enabled: isConnected && !!unauthenticatedAgent && !!principalId,
    agent: unauthenticatedAgent,
    owner: principalId,
  });

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
        </div>

        {!isConnected && (
          <BtnConnectWallet className="hidden xl:block mt-auto w-full" />
        )}
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <div>
          <GradientCard
            className={clsx(
              "px-4 xl:px-8 pt-4 xl:pt-8 pb-8 xl:pb-12",
              "rounded-tr-[inherit]"
            )}
          >
            <TokenHeaderPrice
              className="hidden xl:block mb-8 xl:mb-12"
              token={TOKEN_GLDT}
            />

            <UserTotalStakedAmount position={position} />
          </GradientCard>

          <div className="flex flex-col gap-4 xl:gap-8 pt-8 px-4 pb-4 xl:px-8 xl:pb-8">
            {!isConnected && <DissolveEventsListDisconnected />}
            {isConnected && (
              <div className="rounded-xl border border-orange-500 bg-orange-500/5 text-orange-500 p-4 text-center">
                GLDT staking has been discontinued. Claim any rewards and then
                withdraw your stakes.
              </div>
            )}
            {isConnected && (position.isLoading || position.isError) && (
              <div
                className={clsx(
                  "flex flex-col items-center justify-center gap-4",
                  "border border-border bg-surface-primary",
                  "rounded-xl p-4"
                )}
              >
                <LoaderSpin size="sm" />
                <div>Fetching stake positions...</div>
              </div>
            )}

            {isConnected &&
              position.isSuccess &&
              position.data.is_enable_claiming_rewards && (
                <ClaimRewardsDisclaimer position={position.data} />
              )}

            {isConnected &&
              position.isSuccess &&
              (position.data.total_withdrawable_amount > 0 ||
                position.data.staked_amount > 0) && (
                <div className={styles.container}>
                  <div className="rounded-[inherit] p-4">
                    <div className={styles.title}>
                      Tokens available to withdraw
                    </div>
                    <div className="flex flex-col xl:flex-row justify-between items-center mt-2 gap-4">
                      <div className="flex flex-col items-center xl:items-start shrink-0">
                        <div className={styles.totalAmount}>
                          <NumberToLocaleString
                            value={
                              position.data.staked_amount +
                              position.data.total_withdrawable_amount
                            }
                            decimals={5}
                          />{" "}
                          <span className="text-content/60 font-normal">
                            GLDT
                          </span>
                        </div>
                        <div className={styles.description}>
                          Total of staked amount and already dissolved events.
                        </div>
                      </div>
                      <Withdraw position={position} />
                    </div>
                  </div>
                </div>
              )}
          </div>
        </div>
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Earn;
