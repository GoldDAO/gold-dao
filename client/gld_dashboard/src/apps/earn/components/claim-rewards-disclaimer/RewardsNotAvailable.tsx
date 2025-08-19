const RewardsNotAvailable = () => {
  return (
    <div className="border border-border bg-surface-primary rounded-xl">
      <div className="rounded-[inherit] p-4 bg-surface-secondary/40">
        <div className="text-content/60 text-center xl:text-left">
          No rewards available to claim
        </div>
        <div className="flex flex-col xl:flex-row justify-between items-center mt-2 gap-4">
          <div className="flex flex-col items-center xl:items-start shrink-0">
            <div className="font-semibold text-xl text-content/60">
              Total of: <span>0$</span>
            </div>
            <div className="text-sm text-content/60">
              dispatched in GOLDAO, ICP and OGY
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default RewardsNotAvailable;
