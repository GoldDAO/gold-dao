import { ReactNode } from "react";
import { useParams } from "react-router-dom";

import { LoaderSpin, Logo } from "@components/index";
import { BadgeTransactionKind } from "@components/badges/BadgeTransactionKind";

import useFetchOneTransaction from "@services/ledger/hooks/useFetchOneTransaction";

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

const GLDGovTransactionDetails = () => {
  const params = useParams();

  const { data, isLoading, isSuccess, isError } = useFetchOneTransaction({
    transactionId: params.id as string,
  });

  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Transaction</div>
      </div>

      <section className="pb-16">
        {isSuccess && (
          <>
            <Card title="Type" className="rounded-t-xl">
              <div className="inline-flex">
                <BadgeTransactionKind
                  kind={data.kind as "mint" | "burn" | "approve" | "transfer"}
                />
              </div>
            </Card>
            <Card title="Index">{data.index}</Card>
            <Card title="Timestamp">{data.timestamp}</Card>
            <Card title="From">{data.from_account}</Card>
            <Card title="To">{data.to_account}</Card>
            <Card title="Amount">
              <div className="flex items-center gap-2">
                <Logo name="gldgov" className="h-4 w-4" />
                <div className="font-semibold">{data.amount}</div>
                <div className="text-content/60">GLDGov</div>
              </div>
            </Card>
            <Card title="Fee" className="">
              <div className="flex items-center gap-2">
                <Logo name="gldgov" className="h-4 w-4" />
                <div className="font-semibold">{data.fee}</div>
                <div className="text-content/60">GLDGov</div>
              </div>
            </Card>
            <Card title="Memo" className="rounded-b-xl border-b border-border">
              {data.memo}
            </Card>
          </>
        )}
        {(isLoading || isError) && (
          <div className="flex justify-center py-8">
            <LoaderSpin />
          </div>
        )}
      </section>
    </>
  );
};

export default GLDGovTransactionDetails;
