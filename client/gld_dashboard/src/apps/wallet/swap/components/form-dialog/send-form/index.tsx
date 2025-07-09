import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { Token, TOKENS } from "@shared/utils/tokens";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import ListboxToken from "@wallet/swap/components/form-dialog/listbox-token";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import BalanceAvailable from "../balance-available";
import SendAmountInput from "./send-amount-input";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const SendForm = () => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const balance = useFetchLedgerBalance(
    swapState.token_from.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.token_from.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const onChangeToken = (selectedToken: Token) => {
    dispatchSwapState({ type: "SET_TOKEN_FROM", value: selectedToken });
  };

  return (
    <div className="flex flex-col gap-4 p-4 border border-border rounded-xl bg-surface-secondary">
      <div className="text-copper font-semibold">Send</div>
      <div className="flex justify-between items-start">
        <div>
          <div className="text-2xl">
            <SendAmountInput initialValue={swapState.send_amount_input} />
          </div>
          <div className="text-xs text-content/60">
            {balance.isSuccess ? (
              <div>
                ≈$
                <NumberToLocaleString
                  value={
                    Number(swapState.send_amount_input) * balance.data.price_usd
                  }
                />
              </div>
            ) : (
              <div className="animate-pulse">≈$0</div>
            )}
          </div>
        </div>
        <div>
          <ListboxToken
            value={swapState.token_from.token}
            options={TOKENS}
            optionsDisabled={TOKENS.filter(
              (t) => t.id === swapState.token_to.token.id
            )}
            onChange={onChangeToken}
          ></ListboxToken>
        </div>
      </div>
      <div className="flex justify-between items-center">
        <BalanceAvailable token={swapState.token_from.token} />
        <div className="">Max btn</div>
      </div>
      {/* <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <div className="flex items-center justify-center rounded-full bg-surface-secondary h-6 w-6 shrink-0 aspect-square">
            <Logo name={token.id} />
          </div>
          <input
            id="amount"
            type="text"
            autoComplete="off"
            placeholder={`0 ${token.name}`}
            className={clsx(
              "w-full outline-none focus:outline-none focus:border-none focus:ring-0 bg-surface-secondary",
              "placeholder:text-content/40"
            )}
            {...register("amount", {
              pattern: /[0-9.]/,
              required: "Amount is required",
              validate: {
                isAmountBelowBalance: (v: string) =>
                  isAmountBelowBalance(v) ||
                  "Amount must not exceed your balance minus network fees",
                isAmountAboveFee: (v: string) =>
                  isAmountAboveFee(v) ||
                  "Amount must not be less or equal than transaction fee",
              },
            })}
          />
        </div>
        <button
          onClick={handleOnClickMaxBalance}
          type="button"
          className="rounded-md py-1 px-2 bg-surface-primary text-content/60 border border-border text-xs disabled:cursor-not-allowed cursor-pointer"
          data-tooltip-id="tooltip"
          data-tooltip-html="Max selects your balance minus network fees,<br>ensuring your transaction completes successfully."
        >
          Max
        </button>
      </div>
      <div className="text-content/40 text-sm mt-2 ml-1">
        $
        <NumberToLocaleString
          value={Number(watchedAmount * balance.data.price_usd)}
        />
      </div> */}
    </div>
  );
};

export default SendForm;
