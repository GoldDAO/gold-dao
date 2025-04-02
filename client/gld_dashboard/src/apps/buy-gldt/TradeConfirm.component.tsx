import { useAtom } from "jotai";

import { BuyGLDTStateReducerAtom } from "./atoms";

import { Logo } from "@components/index";
import { Button } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

const ConfirmSwap = () => {
  const [buyAtomState, dispatch] = useAtom(BuyGLDTStateReducerAtom);
  const { pay_token, receive_token } = buyAtomState;

  return (
    <div className="flex flex-col gap-4 lg:gap-8 mt-4 lg:mt-8">
      <div className="rounded-xl bg-surface-secondary border border-border">
        <div className="p-4 lg:p-6 border-b border-border">
          <div className="text-sm mb-4 text-content/60">You pay</div>
          <div className="flex flex-row justify-between items-end">
            <div className="flex items-center gap-2 text-4xl">
              <Logo
                name={pay_token.token.id}
                className="h-10 w-10 shrink-0 aspect-square"
              />
              <TokenValueToLocaleString
                value={pay_token.amount as bigint}
                tokenDecimals={pay_token.decimals as number}
              />
              <div>{pay_token.token.name}</div>
            </div>
            <div className="text-content/60">≈ $todo</div>
          </div>
        </div>
        <div className="p-4 lg:p-6">
          <div className="text-sm mb-4 text-content/60">
            You receive approximately
          </div>
          <div className="flex flex-row justify-between items-end">
            <div className="flex items-center gap-2 text-4xl">
              <Logo
                name={receive_token.token.id}
                className="h-10 w-10 shrink-0 aspect-square"
              />
              <TokenValueToLocaleString
                value={receive_token.amount as bigint}
                tokenDecimals={receive_token.decimals as number}
              />
              <div>{receive_token.token.name}</div>
            </div>
            <div className="text-content/60">≈ $todo</div>
          </div>
        </div>
      </div>

      <div className="rounded-xl border border-border p-4 lg:p-6">
        <div className="mb-4">Transaction details</div>
        <div className="flex flex-col gap-2">
          <div className="flex justify-between items-center">
            <div className="text-content/60">Slippage</div>
            <div className="text-content/60">todo%</div>
          </div>
          <div className="flex justify-between items-center">
            <div className="text-content/60">Fees</div>
            <div>todo</div>
          </div>
          <div className="flex justify-between items-start">
            <div className="text-content/60">Amount received on wallet</div>
            <div className="flex flex-col items-end">
              <div>todo</div>
              <div className="text-content/60">$todo</div>
            </div>
          </div>
        </div>
      </div>

      <Button
        onClick={() => dispatch({ type: "CONFIRM" })}
        className="w-full px-4 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
      >
        <>
          Buy ≈{" "}
          <TokenValueToLocaleString
            value={receive_token.amount}
            tokenDecimals={receive_token.decimals}
            decimals={5}
          />{" "}
          GLDT
        </>
      </Button>
    </div>
  );
};

export default ConfirmSwap;
