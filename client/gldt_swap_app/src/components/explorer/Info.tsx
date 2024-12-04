import { LoaderSpin } from "@components/ui";
import { useLedgerMetadata } from "@hooks/ledger";

export const ExplorerInfo = () => {
  const ledgerMetadata = useLedgerMetadata({
    ledger: "GLDT",
  });

  return (
    <div className="border border-border rounded-xl bg-surface p-4 md:p-6">
      <div className="mb-4 font-semibold">Overview</div>

      {ledgerMetadata.isSuccess && ledgerMetadata.data && (
        <>
          <div className="flex items-center border-b border-border py-4">
            <div className="font-semibold text-content/60 w-40">Name</div>
            <div className="text-content/60">{ledgerMetadata.data.name}</div>
          </div>
          <div className="flex items-center border-b border-border py-4">
            <div className="font-semibold text-content/60 w-40">Symbol</div>
            <div className="text-content/60">{ledgerMetadata.data.symbol}</div>
          </div>
          <div className="flex items-center border-b border-border py-4">
            <div className="font-semibold text-content/60 w-40">Decimals</div>
            <div className="text-content/60">
              {ledgerMetadata.data.decimals}
            </div>
          </div>
          <div className="flex items-center border-b border-border py-4">
            <div className="font-semibold text-content/60 w-40">
              Total Supply
            </div>
            <div className="text-content/60">
              {ledgerMetadata.data.totalSupply.string}
            </div>
          </div>
          <div className="flex items-center border-b border-border py-4">
            <div className="font-semibold text-content/60 w-40">
              Transfer Fee
            </div>
            <div className="text-content/60">
              {ledgerMetadata.data.fee.string} {ledgerMetadata.data.symbol}
            </div>
          </div>
          <div className="flex items-center py-4">
            <div className="font-semibold text-content/60 w-40">Market Cap</div>
            <div className="text-content/60">
              {ledgerMetadata.data.marketCap} $
            </div>
          </div>
        </>
      )}
      {(ledgerMetadata.isLoading || ledgerMetadata.isError) && (
        <div className="flex justify-center pb-4">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};

export default ExplorerInfo;
