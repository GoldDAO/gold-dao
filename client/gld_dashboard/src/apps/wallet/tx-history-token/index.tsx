import { Fragment, useEffect } from "react";
import { useInView } from "react-intersection-observer";
import { useAtomValue } from "jotai";
import clsx from "clsx";
import { ArrowRight } from "iconsax-react";
import { useAuth } from "@auth/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import { Transaction } from "@services/ledger-index/utils/interfaces";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import useFetchAccountTransactions from "@services/ledger-index/hooks/useFetchAccountTransactions";
import Address from "@components/strings/Address";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ListItem = ({ tx }: { tx: Transaction }) => {
  const { unauthenticatedAgent, isConnected } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: token.id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const price = useFetchTokenPrice(unauthenticatedAgent, {
    from: token.name,
    from_canister_id: token.canisterId,
    amount: tx.amount ?? 0n,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const renderAddress = (address: string | undefined) => {
    return address ? <Address>{address}</Address> : "N/A";
  };

  const renderAmount = (amount: bigint | undefined) => {
    if (!decimals.isSuccess) return <div>Loading...</div>;
    if (tx.is_credit) {
      return (
        <div className="text-success">
          +
          <E8sToLocaleString
            value={amount as bigint}
            tokenDecimals={decimals.data}
          />{" "}
          {token.name}
        </div>
      );
    } else {
      return (
        <div className="text-danger">
          -
          <E8sToLocaleString
            value={amount as bigint}
            tokenDecimals={decimals.data}
          />{" "}
          {token.name}
        </div>
      );
    }
  };

  return (
    <div
      className={clsx(
        "p-2 lg:p-4 border border-border rounded-xl",
        "flex items-start justify-between"
      )}
    >
      <div className="flex items-start lg:items-center gap-4">
        <div className="w-24 flex justify-center px-4 py-3 border border-gold/5 bg-gold/10 text-copper text-sm font-semibold rounded-xl">
          {tx.kind}
        </div>
        <div className="text-sm">
          <div className="inline-flex flex-col lg:flex-row lg:items-center gap-1 lg:gap-2">
            <div className="flex gap-2 items-center lg:hidden">
              <div className="text-lg">{renderAmount(tx.amount)}</div>
              <div className="text-content/60 text-sm">
                {price.isSuccess ? (
                  <>
                    $
                    <NumberToLocaleString value={price.data.amount_usd} />
                  </>
                ) : (
                  <div>Loading...</div>
                )}
              </div>
            </div>
            <div className="text-center mb-2 lg:mb-0">
              {renderAddress(tx.from)}
            </div>
            <div className="flex justify-center">
              <ArrowRight size={12} className="rotate-90 lg:rotate-0" />
            </div>
            <div className="text-center">{renderAddress(tx.to)}</div>
          </div>
          <div className="text-content/60 text-sm mt-2 lg:mt-0">
            {tx.timestamp}
          </div>
        </div>
      </div>
      <div className="hidden lg:block">
        <div className="text-right text-lg">{renderAmount(tx.amount)}</div>
        <div className="text-content/60 text-sm text-right">
          {price.isSuccess ? (
            <>
              $
              <NumberToLocaleString value={price.data.amount_usd} />
            </>
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    </div>
  );
};

const List = () => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const { ref, inView } = useInView();
  const token = useAtomValue(TokenSelectedAtom);

  const { status, data, isFetchingNextPage, fetchNextPage, hasNextPage } =
    useFetchAccountTransactions(
      token.canister_id_ledger_index,
      unauthenticatedAgent,
      {
        account: principalId, // "4lxgi-y7rlh-onvu4-jtszk-z67wq-ldekw-rfsp3-yxrjy-dgwsl-zn6tl-eqe"
        enabled: !!unauthenticatedAgent && isConnected,
        ledger: token.id,
      }
    );

  useEffect(() => {
    if (inView) {
      fetchNextPage();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [inView]);

  if (status === "pending") {
    return <div>Loading...</div>;
  }

  if (status === "error") {
    return <div>Error</div>;
  }

  if (status === "success" && data?.pages[0]?.data.length === 0) {
    return (
      <div className="p-4 bg-surface-primary flex justify-center border border-border rounded-xl">
        <div>No transactions found</div>
      </div>
    );
  }

  return (
    <div className="flex flex-col xl:flex-grow xl:h-full gap-2 xl:overflow-y-auto xl:pr-4">
      {data?.pages.map((page) => (
        <Fragment key={page.cursor_index}>
          {page.data.map((tx) => (
            <Fragment key={tx.index}>
              <ListItem tx={tx} />
            </Fragment>
          ))}
        </Fragment>
      ))}
      <div ref={ref}></div>
      <div className="p-4 flex justify-center">
        {isFetchingNextPage ? (
          <div>Loading...</div>
        ) : (
          <div>{!hasNextPage && <div>No more transactions found</div>}</div>
        )}
      </div>
    </div>
  );
};

export default List;
