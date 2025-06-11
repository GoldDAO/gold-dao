import { InfoCircle, ExportSquare, Warning2 } from "iconsax-react";
import { Logo, Button } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import { PayToken, ReceiveToken } from "@buy/shared/utils";

const ConfirmDialog = ({
  open,
  handleClose,
  handleConfirm,
  payToken,
  receiveToken,
  slippage,
  maxSlippage,
  networkFee,
  lpFee,
}: {
  open: boolean;
  handleClose: () => void;
  handleConfirm: () => void;
  payToken: PayToken;
  receiveToken: ReceiveToken;
  slippage: number;
  maxSlippage: number;
  networkFee: bigint;
  lpFee: bigint;
}) => {
  return (
    <Dialog open={open} handleOnClose={handleClose} title="Confirm Purchase">
      <div className="flex flex-col gap-4 mt-4">
        <div className="rounded-xl bg-surface-secondary border border-border">
          <div className="p-4 xl:p-6 border-b border-border">
            <div className="text-sm mb-4 text-content/60">You pay</div>
            <div className="flex flex-row justify-between items-end">
              <div className="flex items-center gap-2 text-4xl">
                <Logo
                  name={payToken.token.id}
                  className="h-10 w-10 shrink-0 aspect-square"
                />
                <TokenValueToLocaleString
                  value={payToken.amount as bigint}
                  tokenDecimals={payToken.decimals as number}
                />
                <div>{payToken.token.name}</div>
              </div>
              <div className="text-content/60">
                ≈ $
                <NumberToLocaleString value={payToken.amount_usd as number} />
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
                data-tooltip-content={`The exact amount of ${receiveToken.token.name} received will vary due to market
                    fluctuations and slippage.`}
              />
            </div>

            <div className="flex flex-row justify-between items-end">
              <div className="flex items-center gap-2 text-4xl">
                <Logo
                  name={receiveToken.token.id}
                  className="h-10 w-10 shrink-0 aspect-square"
                />
                <TokenValueToLocaleString
                  value={receiveToken.amount as bigint}
                  tokenDecimals={receiveToken.decimals as number}
                  decimals={5}
                />
                <div>{receiveToken.token.name}</div>
              </div>
              <div className="text-content/60">
                ≈ $
                <NumberToLocaleString
                  value={receiveToken.amount_usd as number}
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
              <div className="flex items-center gap-1">
                {slippage > maxSlippage && (
                  <Warning2
                    size={20}
                    className="text-amber-500"
                    variant="Bold"
                    data-tooltip-id="tooltip"
                    data-tooltip-html={
                      "Warning: Current slippage is large than the recommended limit of 5%. The price impact of your purchase is quite significant."
                    }
                  />
                )}
                <div className="text-content/60">
                  <NumberToLocaleString value={slippage} decimals={2} />%
                </div>
              </div>
            </div>
            <div className="flex justify-between items-center px-2">
              <div className="flex items-center gap-1">
                <div className="text-content/60">Max slippage</div>
                <InfoCircle
                  size={16}
                  data-tooltip-id="tooltip"
                  data-tooltip-html={
                    "Slippage is the difference between the expected price of a trade and the price at which it's executed.<br />The system will allow slippages up to 5% and will ask you for confirmation if the slippage is higher."
                  }
                />
              </div>

              <div className="text-content/60">{maxSlippage}%</div>
            </div>
            <div>
              <div className="flex justify-between items-center px-2">
                <div className="text-content/60">Fees</div>
                {receiveToken.decimals && networkFee && lpFee ? (
                  <>
                    <TokenValueToLocaleString
                      value={networkFee + lpFee}
                      tokenDecimals={receiveToken.decimals}
                    />{" "}
                    {receiveToken.token.name}
                  </>
                ) : (
                  <div>Loading...</div>
                )}
              </div>
              <div className="mt-4 text-content/60 text-sm flex flex-col gap-4 border border-border rounded-md bg-surface-secondary p-4">
                <div className="flex justify-between items-center">
                  <div>Network fee</div>
                  {receiveToken.decimals && networkFee ? (
                    <>
                      <TokenValueToLocaleString
                        value={networkFee}
                        tokenDecimals={receiveToken.decimals}
                      />{" "}
                      {receiveToken.token.name}
                    </>
                  ) : (
                    <div>Loading...</div>
                  )}
                </div>
                <div className="border-t border-border"></div>
                <div className="flex justify-between items-center">
                  <div>LP fee</div>
                  {receiveToken.decimals && lpFee ? (
                    <>
                      <TokenValueToLocaleString
                        value={lpFee}
                        tokenDecimals={receiveToken.decimals}
                      />{" "}
                      {receiveToken.token.name}
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
          onClick={handleConfirm}
          className="w-full px-4 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
        >
          <>
            Buy ≈{" "}
            <TokenValueToLocaleString
              value={receiveToken.amount}
              tokenDecimals={receiveToken.decimals as number}
              decimals={5}
            />{" "}
            {receiveToken.token.name}
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
              <ExportSquare className="ml-2 h-4 w-4" />
            </a>
          </div>
        </div>
      </div>
    </Dialog>
  );
};

export default ConfirmDialog;
