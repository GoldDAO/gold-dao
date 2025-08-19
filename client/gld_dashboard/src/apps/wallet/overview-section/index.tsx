import clsx from "clsx";
import { useAtomValue } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import GradientCard from "@shared/ui/card/GradientCard";
import TokenHeaderPrice from "@shared/components/token-header-price";
import WalletListMobile from "@wallet/wallet-list-mobile";
import FeaturesBtn from "@wallet/overview-section/FeaturesBtn";
import HeaderNFT from "@wallet/overview-section/HeaderNFT";
import TotalCountUserNFTs from "@shared/components/total-count-user-nfts";
import TotalCountToken from "@shared/components/total-count-token";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";

const OverviewSection = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [searchParams] = useSearchParams();
  const token = useAtomValue(TokenSelectedAtom);

  const balance = useFetchLedgerBalance(
    token.canister_id,
    unauthenticatedAgent,
    {
      ledger: token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const renderUserBalance = () => {
    if (!isConnected) {
      return <TotalCountToken token={token} amount={0} amountUSD={0} />;
    }
    if (balance.isLoading || balance.isError || balance.isFetching) {
      return (
        <TotalCountToken
          token={token}
          isFetching={true}
          amount={0}
          amountUSD={0}
        />
      );
    }
    if (balance.isSuccess && balance.data) {
      return (
        <TotalCountToken
          token={token}
          amount={balance.data.balance}
          amountUSD={balance.data.balance_usd}
        />
      );
    }
    return <TotalCountToken token={token} amount={0} amountUSD={0} />;
  };

  return (
    <GradientCard className="p-4 xl:p-8 relative">
      <div className="flex flex-col items-center">
        <div className="pb-8 xl:pb-16">
          <div className="flex flex-col items-center">
            {searchParams.get("token") === "GLDNFT" ? (
              <>
                <HeaderNFT className="hidden xl:block mb-8 xl:mb-12" />
                <TotalCountUserNFTs />
              </>
            ) : (
              <>
                <TokenHeaderPrice
                  className="hidden xl:block mb-8 xl:mb-12"
                  token={token}
                />
                {renderUserBalance()}
              </>
            )}
          </div>

          <WalletListMobile className="flex justify-center xl:hidden mt-6 mb-8" />
        </div>
      </div>

      <div className={clsx("absolute -bottom-9 left-1/2 -translate-x-1/2")}>
        <FeaturesBtn />
      </div>
    </GradientCard>
  );
};

export default OverviewSection;
