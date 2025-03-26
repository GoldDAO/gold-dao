import clsx from "clsx";
import { useAtom } from "jotai";

import { useAuth } from "@auth/index";

import BuyGLDTStateAtom from "./atoms";

import { Logo } from "@components/index";
import { Button } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";

const ConfirmSwap = ({ className }: { className?: string }) => {
  const { authenticatedAgent, unauthenticatedAgent, isConnected } = useAuth();
  const [buyAtomState, setBuyAtomstate] = useAtom(BuyGLDTStateAtom);
  const {
    pay_token,
    pay_amount,
    receive_amount,
    pay_token_decimals,
    receive_token,
    pay_token_user_balance,
  } = buyAtomState;

  const decimals = useFetchDecimals(pay_token.canisterId, authenticatedAgent, {
    ledger: pay_token.id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const handleConfirmTrade = () => {
    setBuyAtomstate((state) => ({
      ...state,
      is_open_confirm_dialog: false,
      is_open_details_dialog: true,
    }));
  };

  return (
    <div className={className}>
      <div className="flex flex-col gap-4 lg:gap-8 items-center text-center mt-4 lg:mt-8">
        <div
          className={clsx(
            "bg-surface-primary",
            "text-3xl lg:text-6xl font-semibold"
          )}
        >
          <div className="flex items-center justify-center gap-4">
            {receive_amount} GLDT{" "}
            <div className="rounded-full bg-surface-secondary h-10 w-10 lg:h-16 lg:w-16 shrink-0 aspect-square">
              <Logo name="gldt" className="p-1" />
            </div>
          </div>
          <div className="font-semibold text-lg lg:text-2xl mt-1">
            {pay_amount && pay_token_decimals ? (
              <>
                Spend ≈{" "}
                <TokenValueToLocaleString
                  value={pay_amount}
                  tokenDecimals={pay_token_decimals}
                />{" "}
                {pay_token.name}
              </>
            ) : (
              <div>Loading...</div>
            )}
          </div>
        </div>

        <div className="p-8">Transactions Details (todo)</div>

        <div className="w-full">
          <Button
            onClick={handleConfirmTrade}
            disabled={!decimals.isSuccess}
            className="w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
          >
            {decimals.isSuccess ? (
              <>
                Buy ≈{" "}
                <TokenValueToLocaleString
                  value={BigInt(receive_amount * 10 ** decimals.data)}
                  tokenDecimals={decimals.data}
                />{" "}
                {receive_token.name}
              </>
            ) : (
              "Loading..."
            )}
          </Button>

          <div className="mt-4 text-sm lg:text-base">
            {pay_token_user_balance !== null && decimals.isSuccess ? (
              <div className="flex items-center justify-center gap-2">
                <div>
                  Your balance:{" "}
                  <TokenValueToLocaleString
                    value={pay_token_user_balance}
                    tokenDecimals={decimals.data}
                  />{" "}
                  {pay_token.name}
                </div>
                <div className="rounded-full bg-surface-secondary h-8 w-8 shrink-0 aspect-square">
                  <Logo name={pay_token.id} className="p-1" />
                </div>
              </div>
            ) : (
              "Loading..."
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConfirmSwap;
