import { useNavigate } from "react-router-dom";
import { LoaderSpin } from "@components/ui";

import { useFetchLedgerOneAccountSubaccounts } from "@hooks/gldt_ledger_indexer";
import { SelectSubaccount } from "@components/ui/select/index";
import { usePagination } from "@utils/table/useTable";

export const OwnerSubaccounts = ({
  owner,
  subaccount,
  className,
}: {
  owner: string;
  subaccount?: string | undefined;
  className?: string;
}) => {
  const navigate = useNavigate();
  const [pagination] = usePagination();

  const { data, isSuccess, isLoading, isError } =
    useFetchLedgerOneAccountSubaccounts({
      owner,
    });

  const handleOnChange = (subaccount: string) => {
    let path: string;
    if (subaccount !== "Default subaccount")
      path = `/explorer/account?owner=${owner}&subaccount=${subaccount}&page_size=${pagination.pageSize}&page_index=0`;
    else
      path = `/explorer/account?owner=${owner}&page_size=${pagination.pageSize}&page_index=0`;
    navigate(path, { replace: true });
  };

  return (
    <div
      className={`border border-border rounded-xl bg-surface p-6 ${className}`}
    >
      <div className="text-center lg:text-left mb-4">Subaccount</div>
      {isSuccess && data.length && (
        <SelectSubaccount
          options={data}
          handleOnChange={(v) => handleOnChange(v)}
          value={subaccount ?? "Default subaccount"}
          disabled={data.length === 1 && data[0] === "Default subaccount"}
        />
      )}
      {(isLoading || isError) && (
        <div className="flex justify-center">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};
