// import { useEffect } from "react";

import { useTransfer, Token } from "@context/transfer";
import { TransferProceedLedgerProvider } from "@context/transfer/proceed-ledger";

import SelectTokenType from "./select/TokenType";
import TransferLedger from "./transfer/TransferLedger";

const Transfer = () => {
  const { state: transferState } = useTransfer();
  const { token } = transferState;

  return (
    <div className="mt-8">
      <div className="flex justify-between items-center p-4 border border-border rounded-xl">
        <div className="text-gold font-semibold text-sm">Type of transfer</div>
        <SelectTokenType className="" />
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
    </div>
  );
};

export default Transfer;
