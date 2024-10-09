const Balance = ({ ledger, balance }: { ledger: string; balance: number }) => {
  return (
    <div className="inline-flex justify-start items-center px-2 py-1 bg-surface-2 text-content/60 text-xs rounded-lg">
      <div>Your balance: </div>
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

export default Balance;
