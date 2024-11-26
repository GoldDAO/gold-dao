import CopyToClipboard from "@components/shared/button/CopyToClipboard";

export const AccountOwner = ({
  owner,
  className,
}: {
  owner: string;
  className?: string;
}) => {
  return (
    <div
      className={`border border-border rounded-xl bg-surface p-6 ${className}`}
    >
      <div className="text-center lg:text-left mb-4">Owner</div>

      <div className="w-full flex items-center justify-center lg:justify-start">
        <button
          data-tooltip-id="tooltip"
          data-tooltip-content={owner}
          className="mr-2 truncate text-2xl font-semibold"
        >
          {owner}
        </button>
        <CopyToClipboard value={owner} />
      </div>
    </div>
  );
};
