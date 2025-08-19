import clsx from "clsx";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import GradientCard from "@shared/ui/card/GradientCard";
import TotalCountUserNFTs from "@shared/components/total-count-user-nfts";
import FeaturesBtn from "@advanced/gldt/overview-section/features-btn";
import TotalCountToken from "@shared/components/total-count-token";
import { TOKEN_GLDT } from "@shared/utils/tokens";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";

const OverviewSection = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();

  const balance = useFetchLedgerBalance(
    TOKEN_GLDT.canister_id,
    unauthenticatedAgent,
    {
      ledger: TOKEN_GLDT.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const renderUserBalance = () => {
    if (!isConnected) {
      return <TotalCountToken token={TOKEN_GLDT} amount={0} amountUSD={0} />;
    }
    if (balance.isLoading || balance.isError || balance.isFetching) {
      return (
        <TotalCountToken
          token={TOKEN_GLDT}
          isFetching={true}
          amount={0}
          amountUSD={0}
        />
      );
    }
    if (balance.isSuccess && balance.data) {
      return (
        <TotalCountToken
          token={TOKEN_GLDT}
          amount={balance.data.balance}
          amountUSD={balance.data.balance_usd}
        />
      );
    }
    return <TotalCountToken token={TOKEN_GLDT} amount={0} amountUSD={0} />;
  };

  return (
    <GradientCard className="p-4 xl:p-8 relative">
      <div className="flex flex-col items-center">
        <div className="pb-16">
          <div className="flex flex-col gap-2 items-center mb-8 xl:mb-12">
            <div className="flex items-center gap-2">
              <Logo name="gldt" className="h-10 w-10" />
              <div>
                <div>GLDT</div>
                <div className="text-content/60 text-sm">Mint & Burn</div>
              </div>
            </div>
          </div>

          <div className="flex flex-col xl:flex-row xl:items-stretch gap-2 xl:gap-8 justify-center items-center">
            <TotalCountUserNFTs />
            <div className="xl:border-l border-border h-auto my-2" />
            {renderUserBalance()}
          </div>
        </div>
      </div>
      <div className={clsx("absolute -bottom-9 left-1/2 -translate-x-1/2")}>
        <FeaturesBtn />
      </div>
    </GradientCard>
  );
};

export default OverviewSection;
