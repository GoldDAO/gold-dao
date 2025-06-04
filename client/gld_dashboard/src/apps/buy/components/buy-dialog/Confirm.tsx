import { useAtom } from "jotai";
import { InfoCircle } from "iconsax-react";
import { BuyGLDTStateReducerAtom } from "@buy/atoms/BuyGLDTAtom";
import { Logo } from "@components/index";
import { Button } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import { ArrowTopRightOnSquareIcon } from "@heroicons/react/20/solid";

const Confirm = () => {
  const [buyAtomState, dispatch] = useAtom(BuyGLDTStateReducerAtom);
  const {
    pay_token,
    receive_token,
    slippage,
    network_fee,
    max_slippage,
    lp_fee,
  } = buyAtomState;

  return (
    <div className="flex flex-col gap-4 mt-4">
      <div className="rounded-xl bg-surface-secondary border border-border">
        <div className="p-4 xl:p-6 border-b border-border">
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
            <div className="text-content/60">
              ≈ $
              <NumberToLocaleString value={pay_token.amount_usd as number} />
            </div>
          </div>
        </div>
        <div className="p-4 xl:p-6">
          <div className="flex items-center gap-2 mb-4">
            <div className="text-sm text-content/60">
              You receive approximately
            </div>

            <InfoCircle
              size={16}
              data-tooltip-id="tooltip"
              data-tooltip-content="The exact amount of GLDT received will vary due to market
                    fluctuations and slippage."
            />
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
                decimals={5}
              />
              <div>{receive_token.token.name}</div>
            </div>
            <div className="text-content/60">
              ≈ $
              <NumberToLocaleString
                value={receive_token.amount_usd as number}
              />
            </div>
          </div>
        </div>
      </div>

      <div className="rounded-xl border border-border p-4 xl:p-6">
        <div className="mb-4">Transaction details</div>
        <div className="flex flex-col gap-4">
          <div className="flex justify-between items-center px-2">
            <div className="text-content/60">Slippage</div>
            <div className="text-content/60">{slippage}%</div>
          </div>
          <div className="flex justify-between items-center px-2">
            <div className="text-content/60">
              Max slippage (todo tooltip info)
            </div>
            <div className="text-content/60">{max_slippage}%</div>
          </div>
          <div>
            <div className="flex justify-between items-center px-2">
              <div className="text-content/60">Fees</div>
              {receive_token.decimals && network_fee && lp_fee ? (
                <>
                  <TokenValueToLocaleString
                    value={network_fee + lp_fee}
                    tokenDecimals={receive_token.decimals}
                  />{" "}
                  {receive_token.token.name}
                </>
              ) : (
                <div>Loading...</div>
              )}
            </div>
            <div className="mt-4 text-content/60 text-sm flex flex-col gap-4 border border-border rounded-md bg-surface-secondary p-4">
              <div className="flex justify-between items-center">
                <div>Network fee</div>
                {receive_token.decimals && network_fee ? (
                  <>
                    <TokenValueToLocaleString
                      value={network_fee}
                      tokenDecimals={receive_token.decimals}
                    />{" "}
                    {receive_token.token.name}
                  </>
                ) : (
                  <div>Loading...</div>
                )}
              </div>
              <div className="border-t border-border"></div>
              <div className="flex justify-between items-center">
                <div>LP fee</div>
                {receive_token.decimals && lp_fee ? (
                  <>
                    <TokenValueToLocaleString
                      value={lp_fee}
                      tokenDecimals={receive_token.decimals}
                    />{" "}
                    {receive_token.token.name}
                  </>
                ) : (
                  <div>Loading...</div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      <Button
        onClick={() => dispatch({ type: "CONFIRM" })}
        className="w-full px-4 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
      >
        <>
          Buy ≈{" "}
          <TokenValueToLocaleString
            value={receive_token.amount}
            tokenDecimals={receive_token.decimals as number}
            decimals={5}
          />{" "}
          GLDT
        </>
      </Button>

      <div className="flex justify-center">
        <div className="flex items-center gap-1 text-content/60 text-sm">
          In partnership with
          <a
            href="https://www.kongswap.io/"
            target="_blank"
            rel="noopener noreferrer"
            className={`flex items-center text-content hover:text-primary`}
          >
            <div>KongSwap</div>
            <ArrowTopRightOnSquareIcon className="ml-2 h-5 w-5" />
          </a>
        </div>
      </div>
    </div>
  );
};

export default Confirm;
