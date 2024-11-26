import { useSearchParams } from "react-router-dom";
import { LoaderSpin } from "@components/ui";

import { useFetchLedgerOneAccountSubaccounts } from "@hooks/gldt_ledger_indexer";
import { SelectSubaccount } from "@components/ui/select/index";

export const OwnerSubaccounts = ({
  owner,
  subaccount,
  className,
}: {
  owner: string;
  subaccount?: string | undefined;
  className?: string;
}) => {
  const [searchParams, setSearchParams] = useSearchParams();

  const { data, isSuccess, isLoading, isError } =
    useFetchLedgerOneAccountSubaccounts({
      owner,
    });

  const handleOnChange = (subaccount: string) => {
    searchParams.set("subaccount", subaccount);
    setSearchParams(searchParams, { replace: true });
  };

  return (
    <div
      className={`border border-border rounded-xl bg-surface p-6 ${className}`}
    >
      <div className="text-center lg:text-left mb-4">Subaccount</div>
      {isSuccess &&
        data &&
        (Number(data) !== 0 ? (
          <div>
            <SelectSubaccount
              options={data}
              handleOnChange={(v) => handleOnChange(v)}
              value={subaccount ?? ""}
            />
          </div>
        ) : (
          <div className="font-semibold">No subaccount</div>
        ))}
      {(isLoading || isError) && (
        <div className="flex justify-center">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};
