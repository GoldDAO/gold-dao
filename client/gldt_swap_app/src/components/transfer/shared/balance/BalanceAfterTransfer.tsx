const BalanceAfterTransfer = ({
  ledger,
  balance,
}: {
  ledger: string;
  balance: number;
}) => {
  return (
    <div className="flex items-center justify-between border border-border bg-surface-2 p-4 rounded-xl">
      <div>Balance after transfer</div>
      <div className="flex items-center font-semibold">
        <img
          className="mx-2 h-4 w-4"
          src={`/${ledger.toLocaleLowerCase()}_logo.svg`}
          alt={`${ledger} Logo`}
        />
        <span>
          {balance} {ledger}
        </span>
      </div>
    </div>
  );
};

export default BalanceAfterTransfer;
