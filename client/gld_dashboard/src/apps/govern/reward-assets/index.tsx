import { useAuth } from "@auth/index";
import { Token, TokensList } from "../utils";
import useGetTokenTotalStakedAmount from "../utils/useGetTokenTotalStakedAmount";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { Logo } from "@components/logos";

const RewardAssetItem = ({ token }: { token: Token }) => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();

  const stakedAmount = useGetTokenTotalStakedAmount({
    canisterIdLedger: token.canisterId,
    owner: principalId,
    agent: unauthenticatedAgent,
    enabled: !!unauthenticatedAgent && isConnected && !!principalId,
  });

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: token.id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  return (
    <div className="flex flex-col items-center p-4 lg:p-2">
      {!stakedAmount.isSuccess || !decimals.isSuccess ? (
        <div>Loading...</div>
      ) : (
        <div className="flex items-center gap-2">
          <Logo name={token.id} className="h-4" />
          <div className="text-2xl">
            <TokenValueToLocaleString
              value={stakedAmount.data}
              decimals={decimals.data}
            />
          </div>

          <div className="text-lg text-content/60">{token.name}</div>
        </div>
      )}

      <div className="text-content/60 text-sm">$0</div>
    </div>
  );
};

const RewardAssets = () => {
  return (
    <div className="grid grid-cols-1 lg:grid-cols-4 border border-border rounded-xl p-2 lg:p-4 bg-surface-primary">
      {TokensList.map((token) => (
        <div
          className="border-b lg:border-r lg:border-b-0 border-border last:border-0"
          key={token.id}
        >
          <RewardAssetItem token={token} />
        </div>
      ))}
    </div>
  );
};
export default RewardAssets;
