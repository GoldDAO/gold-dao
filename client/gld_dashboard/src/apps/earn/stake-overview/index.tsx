import { useState } from "react";
import clsx from "clsx";
import { InfoCircle } from "iconsax-react";
import {
  GLDT_VALUE_1G_NFT,
  GLDT_LEDGER_CANISTER_ID,
  GLDT_STAKE_CANISTER_ID,
} from "@constants";
import { useAuth } from "@auth/index";
import { Button, Logo } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import useFetchUserTotalStaked from "@services/gldt_stake/hooks/useFetchUserTotalStaked";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import useFetchStakeAPY from "@services/gldt_stake/hooks/useFetchStakeAPY";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import GradientCard from "@shared/components/ui/card/GradientCard";

const StakeOverview = () => {
  const { authenticatedAgent, unauthenticatedAgent, isConnected } = useAuth();
  const [isOpenInfoAPYDialog, setIsOpenInfoAPYDialog] = useState(false);

  const fetchUserTotalStaked = useFetchUserTotalStaked(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled: isConnected && !!authenticatedAgent,
    }
  );

  const tokenPrice = useFetchTokenPrice(unauthenticatedAgent, {
    from: "GLDT",
    from_canister_id: GLDT_LEDGER_CANISTER_ID,
    amount: fetchUserTotalStaked.data ?? 0n,
    enabled:
      !!unauthenticatedAgent && isConnected && fetchUserTotalStaked.isSuccess,
  });

  const fetchStakeAPY = useFetchStakeAPY(
    GLDT_STAKE_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );

  const renderTotalActiveStake = () => {
    if (isConnected) {
      if (tokenPrice.isSuccess) {
        return (
          <>
            <TokenValueToLocaleString
              value={tokenPrice.data.amount}
              tokenDecimals={tokenPrice.data.decimals}
            />
            <div className="text-content/60 font-normal">GLDT</div>
          </>
        );
      } else {
        return "Loading...";
      }
    } else {
      return (
        <>
          0<div className="text-content/60 font-normal">GLDT</div>
        </>
      );
    }
  };

  const renderTotalActiveStakePrice = () => {
    if (isConnected) {
      if (tokenPrice.isSuccess) {
        return (
          <>
            <TokenValueToLocaleString
              value={tokenPrice.data.amount / BigInt(GLDT_VALUE_1G_NFT)}
              tokenDecimals={tokenPrice.data.decimals}
            />{" "}
            grams of Gold ({" "}
            <span>
              $<NumberToLocaleString value={tokenPrice.data.amount_usd} />
            </span>
            )
          </>
        );
      } else {
        return "Loading...";
      }
    } else {
      return <>0 grams of Gold ($0)</>;
    }
  };

  return (
    <>
      <GradientCard
        className={clsx(
          "px-4 xl:px-8 pt-4 xl:pt-8 pb-24",
          "rounded-tr-[inherit]"
        )}
      >
        <div className="flex flex-col items-center">
          <div className="grid grid-cols-1 xl:grid-cols-3 w-full">
            <div></div>
            <div className="flex justify-center items-center gap-2">
              <Logo name="gldt" className="h-8 w-8" />
              <div>GLDT</div>
            </div>
            <div className="flex justify-center items-center xl:justify-end mt-2 xl:mt-0">
              <div className="flex items-center px-4 py-1 text-sm bg-secondary text-white/90 rounded-full">
                Current APY:{" "}
                <span>
                  {fetchStakeAPY.isSuccess ? (
                    <>
                      <NumberToLocaleString
                        value={fetchStakeAPY.data}
                        decimals={1}
                      />
                      {"%"}
                    </>
                  ) : (
                    "Loading..."
                  )}
                </span>
                <InfoCircle
                  size={16}
                  className="inline cursor-pointer ml-2"
                  onClick={() => setIsOpenInfoAPYDialog(true)}
                />
              </div>
            </div>
          </div>
          <div className="py-8">
            <div className="flex flex-col items-center gap-2">
              <div>
                <div className="font-semibold flex flex-col items-center gap-2">
                  <div>Total active stake</div>
                  <div className="text-2xl xl:text-4xl  flex items-center gap-2">
                    {renderTotalActiveStake()}
                  </div>
                </div>
              </div>
              <div className="text-sm text-content/60">
                {renderTotalActiveStakePrice()}
              </div>
            </div>
          </div>
        </div>
      </GradientCard>
      <Dialog
        open={isOpenInfoAPYDialog}
        handleOnClose={() => setIsOpenInfoAPYDialog(false)}
      >
        <div className="p-4 text-center">
          <div className="font-semibold text-lg mb-4">Last weeks APY</div>
          <div className="text-content/60 mb-8">
            The tokens are locked for 1 week without rewards before they can be
            withdrawn.
          </div>
          <div className="flex justify-end">
            <Button
              className="px-6 py-2 bg-secondary text-white rounded-full"
              onClick={() => setIsOpenInfoAPYDialog(false)}
            >
              Close
            </Button>
          </div>
        </div>
      </Dialog>
    </>
  );
};

export default StakeOverview;
