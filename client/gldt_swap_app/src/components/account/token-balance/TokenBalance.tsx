import BalanceOGY from "@components/account/token-balance/balanceOGY/BalanceOGY";
import BalanceGLDT from "@components/account/token-balance/balanceGLDT/BalanceGLDT";

const TokenBalance = ({ className }: { className?: string }) => {
  return (
    <div className={className}>
      <div className="border border-border bg-surface-1 p-6 rounded-xl">
        <div className="flex justify-between items-center mb-2">
          <div className="mb-4">Token balance</div>
          <div></div>
        </div>
        <div className="flex flex-col sm:flex-row sm:items-center gap-4">
          <BalanceGLDT />
          <BalanceOGY />
        </div>
      </div>
    </div>
  );
};

export default TokenBalance;
