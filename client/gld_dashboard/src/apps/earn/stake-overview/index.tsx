import {
  GLDT_VALUE_1G_NFT,
  GLDT_LEDGER_CANISTER_ID,
  GLDT_STAKE_CANISTER_ID,
} from "@constants";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import useFetchUserTotalStaked from "@services/gldt_stake/hooks/useFetchUserTotalStaked";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";

const StakeOverview = ({ className }: { className?: string }) => {
  const { authenticatedAgent, unauthenticatedAgent, isConnected } = useAuth();

  const decimals = useFetchDecimals(
    GLDT_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "gldt",
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const fetchUserTotalStaked = useFetchUserTotalStaked(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled: isConnected && !!authenticatedAgent,
    }
  );

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex flex-col gap-2 items-center">
          <div className="flex items-center gap-2">
            <Logo name="gldt" className="h-10 w-10" />
            <div>GLDT</div>
          </div>
        </div>
        <div className="py-8">
          <div className="flex flex-col items-center gap-2">
            <div>
              <div className="font-semibold flex flex-col items-center gap-2">
                <div>Total active stake</div>
                <div className="text-2xl lg:text-4xl  flex items-center gap-2">
                  {fetchUserTotalStaked.isSuccess && decimals.isSuccess ? (
                    <>
                      <TokenValueToLocaleString
                        value={fetchUserTotalStaked.data}
                        tokenDecimals={decimals.data}
                      />
                      <div className="text-content/60 font-normal">GLDT</div>
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
              </div>
            </div>
            <div>
              {fetchUserTotalStaked.isSuccess && decimals.isSuccess ? (
                <div>
                  <TokenValueToLocaleString
                    value={
                      fetchUserTotalStaked.data / BigInt(GLDT_VALUE_1G_NFT)
                    }
                    tokenDecimals={decimals.data}
                  />{" "}
                  grams of Gold ($todo)
                </div>
              ) : (
                <div>Loading...</div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default StakeOverview;
