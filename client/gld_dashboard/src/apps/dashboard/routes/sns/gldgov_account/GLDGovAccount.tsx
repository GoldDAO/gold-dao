import { ReactNode } from "react";
import { useParams } from "react-router-dom";
import useFetchAccount from "@services/ledger/hooks/useFetchAccount";
import { LoaderSpin } from "@components/index";
import { GLDGovTransactionsAccountTable } from "./transactions/table/GLDGovTransactionsAccountTable";
import { usePagination } from "@utils/table/useTable";

import { GOLDAO_LEDGER_CANISTER_ID_IC } from "@constants";

const Card = ({
  title,
  children,
  className,
}: {
  title: string;
  children: ReactNode;
  className?: string;
}) => {
  return (
    <div
      className={`bg-surface-primary/40 border-x border-t border-border ${className}`}
    >
      <div className="px-6 pt-6 pb-8 text-center md:text-left">
        <div className="font-semibold text-sm text-content/60">{title}</div>

        <div className="mt-3 font-semibold">{children}</div>
      </div>
    </div>
  );
};

const GLDGovAccount = () => {
  const params = useParams();
  const [paginationTxs, setPaginationTxs] = usePagination({
    pageIndex: 0,
    pageSize: 10,
  });

  const {
    data: account,
    isSuccess: isSuccessAccount,
    isLoading: isLoadingAccount,
    isError: isErrorAccount,
  } = useFetchAccount(GOLDAO_LEDGER_CANISTER_ID_IC, {
    accountId: params.id as string,
  });

  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">GLDGov</div>
        <div className="text-4xl">Account</div>
      </div>

      <section className="grid grid-cols-1 md:grid-cols-2 mb-6">
        <div>
          {(isLoadingAccount || isErrorAccount) && (
            <div className="flex justify-center items-center">
              <LoaderSpin />
            </div>
          )}
          {isSuccessAccount && (
            <>
              <Card title="ID" className="rounded-t-xl">
                {account?.id}
              </Card>
              <Card title="Owner" className="">
                {account?.owner}
              </Card>
              <Card title="Subaccount" className="">
                {account?.subaccount}
              </Card>
              <Card title="Balance" className="">
                {account?.balance}
              </Card>
              <Card
                title="Total Transactions"
                className="rounded-b-xl border-b border-border"
              >
                {account?.total_transactions}
              </Card>
            </>
          )}
        </div>
      </section>

      <section className="rounded-xl bg-surface-primary/40 mb-6">
        <div className="px-6 pt-6 mb-4 lg:mb-6">
          <h6 className="text-lg font-semibold">GLDGov Transactions</h6>
        </div>
        <GLDGovTransactionsAccountTable
          account={params.id as string}
          pagination={paginationTxs}
          setPagination={setPaginationTxs}
        />
      </section>
    </>
  );
};

export default GLDGovAccount;
