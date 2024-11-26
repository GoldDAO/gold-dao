import { useSearchParams, useParams, useNavigate } from "react-router-dom";
import { BugAntIcon, ArrowUturnLeftIcon } from "@heroicons/react/24/solid";

import { LoaderSpin } from "@components/ui";
import { BadgeTransactionType } from "@components/shared/badge/TransactionType";
import CopyToClipboard from "@components/shared/button/CopyToClipboard";

import NavbarHome from "@components/shared/navbars/Home";

import { useFetchLedgerOneAccountTransaction } from "@hooks/gldt_ledger_indexer/useFetchLedgerOneAccountTransaction";
import { LogoGLDT } from "@components/shared/logos";

export const AccountTransactionDetails = () => {
  const navigate = useNavigate();
  const params = useParams();
  const [searchParams] = useSearchParams();
  // const [pagination, setPagination] = usePagination();
  const owner = searchParams.get("owner") as string;
  const subaccount = searchParams.get("subaccount") as string | undefined;
  const start = params.index ? Number(params.index) + 1 : undefined;

  const { data, isSuccess, isLoading, isError, error } =
    useFetchLedgerOneAccountTransaction({
      pageSize: 1,
      start,
      owner,
      subaccount,
    });

  const handleGoBack = () => {
    const windowHS = window.history.state;

    if (windowHS && windowHS.idx > 0) navigate(-1);
    else navigate("/explorer");
  };

  return (
    <>
      <div className="bg-surface-2">
        <NavbarHome />
        <section className="container mx-auto max-w-4xl px-4 py-8 xl:py-16">
          <div
            className="cursor-pointer flex items-center gap-2"
            onClick={handleGoBack}
          >
            <ArrowUturnLeftIcon className="h-5 w-5" />
            <div>Go back</div>
          </div>
          <div className="my-8">
            <div className="text-4xl font-semibold text-gold">GLDT</div>
            <div className="text-4xl">Transaction Details</div>
          </div>
          <div className="mt-16">
            <div className="border border-border rounded-xl bg-surface p-4 md:p-6">
              {isLoading && (
                <div className="flex justify-center">
                  <LoaderSpin />
                </div>
              )}
              {isSuccess && (
                <>
                  <div className="flex justify-between items-center border-b border-border py-4">
                    <div className="font-semibold text-content/60">Index</div>
                    <div className="text-content/60">{data.index}</div>
                  </div>
                  <div className="flex justify-between items-center border-b border-border py-4">
                    <div className="font-semibold text-content/60">Type</div>
                    <BadgeTransactionType type={data.type as string} />
                  </div>
                  <div className="flex justify-between items-center border-b border-border py-4">
                    <div className="font-semibold text-content/60">
                      Date/Hour
                    </div>
                    <div className="text-content/60">{data.date}</div>
                  </div>
                  <div className="flex justify-between items-center pt-4">
                    <div className="font-semibold text-content/60">Amount</div>
                    <div className="flex items-center gap-2">
                      <div className="font-semibold">{data.amount}</div>
                      <LogoGLDT className="w-4 h-4" />
                    </div>
                  </div>
                  <div className="flex justify-between items-center border-b border-border pt-2 pb-4">
                    <div className="text-content/60">Fee</div>
                    <div className="font-semibold">
                      {data.fee ? (
                        <div className="flex items-center gap-2">
                          <div className="font-semibold">{data.amount}</div>
                          <LogoGLDT className="w-4 h-4" />
                        </div>
                      ) : (
                        "-"
                      )}
                    </div>
                  </div>

                  <div className="flex justify-between items-center border-b border-border py-4 break-all">
                    <div className="font-semibold text-content/60">From</div>
                    <div className="max-w-48 lg:max-w-96">
                      <div className="flex items-center truncate">
                        {data.from?.full !== "Minting account" ? (
                          <div className="flex ml-8 items-center truncate">
                            <div
                              className="truncate text-content/60"
                              data-tooltip-id="tooltip"
                              data-tooltip-content={data.from?.full}
                            >
                              {data.from?.full}
                            </div>
                            <CopyToClipboard value={data.from?.full} />
                          </div>
                        ) : (
                          <div className="text-content/60">
                            {data.from?.full}
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                  <div className="flex justify-between items-center border-b border-border py-4 break-all">
                    <div className="font-semibold text-content/60">To</div>
                    <div className="max-w-48 lg:max-w-96">
                      <div className="flex items-center truncate">
                        {data.to?.full !== "Minting account" ? (
                          <div className="flex ml-8 items-center truncate">
                            <div
                              className="truncate text-content/60"
                              data-tooltip-id="tooltip"
                              data-tooltip-content={data.to?.full}
                            >
                              {data.to?.full}
                            </div>
                            <CopyToClipboard value={data.to?.full} />
                          </div>
                        ) : (
                          <div className="text-content/60">{data.to?.full}</div>
                        )}
                      </div>
                    </div>
                  </div>
                  <div className="flex justify-between items-center py-4">
                    <div className="font-semibold text-content/60">Memo</div>
                    <div className="text-content/60">{data.memo}</div>
                  </div>
                </>
              )}
              {isError && (
                <div className="flex flex-col justify-center items-center">
                  <div>
                    <BugAntIcon className="size-16 mb-6 text-gold/80 animate-bounce" />
                  </div>
                  <div>{error.message}</div>
                </div>
              )}
            </div>
          </div>
        </section>
      </div>
    </>
  );
};
