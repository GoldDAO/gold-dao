import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { Token, TOKENS } from "@shared/utils/tokens";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import ListboxToken from "@wallet/swap/components/form-dialog/listbox-token";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import BalanceAvailable from "@shared/components/BalanceAvailable";
import SendAmountInput from "./send-amount-input";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import MaxButton from "@shared/components/MaxButton";

const SendForm = () => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const balance = useFetchLedgerBalance(
    swapState.send_token.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.send_token.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const onChangeToken = (selectedToken: Token) => {
    dispatchSwapState({ type: "SET_TOKEN_FROM", value: selectedToken });
  };

  const onClickMaxBalance = (amount: string) => {
    dispatchSwapState({
      type: "SET_SEND_AMOUNT",
      value: amount,
    });
  };

  return (
    <div className="flex flex-col gap-4 p-4 border border-border rounded-xl bg-surface-secondary">
      <div className="text-copper font-semibold">Send</div>
      <div className="flex justify-between items-start">
        <div>
          <div className="text-2xl">
            <SendAmountInput />
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
            value={swapState.send_token.token}
            options={TOKENS}
            optionsDisabled={TOKENS.filter(
              (t) => t.id === swapState.receive_token.token.id
            )}
            onChange={onChangeToken}
          ></ListboxToken>
        </div>
      </div>
      <div className="flex justify-between items-center">
        <div className="text-sm text-content/80">
          <BalanceAvailable
            token={swapState.send_token.token.name}
            balance={balance.data?.balance}
          />
        </div>
        <MaxButton
          balance={balance.data?.balance_e8s}
          fee={balance.data?.fee_e8s}
          decimals={balance.data?.decimals}
          handleOnClick={(amount) => onClickMaxBalance(amount)}
        />
      </div>
    </div>
  );
};

export default SendForm;
