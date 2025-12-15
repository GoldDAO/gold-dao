import clsx from "clsx";

const DissolveEventsListDisconnected = () => {
  return (
    <div className="flex flex-col gap-4 relative">
      {[...Array(4)].map((_, index) => (
        <div key={index}>
          <div
            className={clsx(
              "@container",
              "shrink-0",
              "rounded-md xl:rounded-xl border border-border/40 p-4"
            )}
          >
            <div className="flex justify-between items-center p-2">
              <div className="flex items-center gap-2">
                <div className="h-5 w-5 bg-surface-secondary rounded-full" />
                <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
              </div>
              <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
            </div>
          </div>
        </div>
      ))}
      <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-background to-transparent" />
    </div>
  );
};

export default DissolveEventsListDisconnected;
