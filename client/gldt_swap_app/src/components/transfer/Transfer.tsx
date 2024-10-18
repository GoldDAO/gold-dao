import { useTransfer, Token } from "@context/transfer";
import { TransferProceedLedgerProvider } from "@context/transfer/proceed-ledger";
import { TransferProceedNftProvider } from "@context/transfer/proceed-nft";
import { NftProvider } from "@context/index";

import SelectToken from "./shared/select/Token";
import TransferLedger from "./ledger/Ledger";
import TransferNft from "./nft/Nft";

const Transfer = () => {
  const { state: transferState } = useTransfer();
  const { token } = transferState;

  return (
    <div className="mt-8">
      <div className="flex flex-col sm:flex-row justify-between items-center p-4 border border-border rounded-xl">
        <div className="text-gold font-semibold text-sm mb-2 sm:mb-0 ">
          Type of transfer
        </div>
        <SelectToken />
      </div>
      {token === Token.GLDT && (
        <TransferProceedLedgerProvider ledger="GLDT">
          <TransferLedger />
        </TransferProceedLedgerProvider>
      )}
      {token === Token.OGY && (
        <TransferProceedLedgerProvider ledger="OGY">
          <TransferLedger />
        </TransferProceedLedgerProvider>
      )}
      {token === Token.GLD_NFT && (
        <NftProvider>
          <TransferProceedNftProvider>
            <TransferNft />
          </TransferProceedNftProvider>
        </NftProvider>
      )}
    </div>
  );
};

export default Transfer;
