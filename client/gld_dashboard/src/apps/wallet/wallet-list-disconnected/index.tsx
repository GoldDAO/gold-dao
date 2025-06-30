import clsx from "clsx";

const WalletListDisconnected = () => {
  return (
    <div className="flex flex-col gap-2 relative">
      {[...Array(2)].map((_, index) => (
        <div key={index}>
          <div
            className={clsx(
              "lg:@container",
              "shrink-0",
              "rounded-xl border border-surface-secondary p-4 cursor-pointer"
            )}
          >
            <div className="flex justify-between items-center p-2">
              <div className="flex items-center gap-2">
                <div className="h-10 w-10 bg-surface-secondary rounded-full" />
                <div className="flex flex-col gap-1">
                  <div className="h-5 w-[16cqw] bg-surface-secondary rounded-sm" />
                  <div className="h-4 w-[20cqw] bg-surface-secondary rounded-sm" />
                </div>
              </div>
              <div className="flex flex-col gap-1 items-end">
                <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
                <div className="h-4 w-[16cqw] bg-surface-secondary rounded-sm" />
              </div>
            </div>
          </div>
        </div>
      ))}
      <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-surface-primary to-transparent" />
    </div>
  );
};

export default WalletListDisconnected;
