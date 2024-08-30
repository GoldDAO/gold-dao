const GldtAmount = ({
  gldtAmount,
  className,
}: {
  gldtAmount: number;
  className?: string;
}) => {
  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-4">
        <div className="mb-2 font-light text-content/60 text-center sm:text-left">
          GLDT amount
        </div>
        <div className="flex items-center justify-center sm:justify-start gap-2">
          <img className="flex-none h-8" src={`/gldt_logo.svg`} />
          <div className="font-semibold text-4xl">{gldtAmount}</div>
          <div className="font-semibold text-xl">GLDT</div>
        </div>
      </div>
    </div>
  );
};

export default GldtAmount;
