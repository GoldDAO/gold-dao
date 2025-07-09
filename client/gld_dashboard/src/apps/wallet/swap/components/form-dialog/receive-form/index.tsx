import { useAtom } from "jotai";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { useAuth } from "@auth/index";
import { Token, TOKENS } from "@shared/utils/tokens";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchSwapAmount from "@shared/hooks/useFetchSwapAmount";
import ListboxToken from "@wallet/swap/components/form-dialog/listbox-token";
import BalanceAvailable from "../balance-available";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ReceiveForm = () => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const balance = useFetchLedgerBalance(
    swapState.token_to.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.token_to.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const swapAmount = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: swapState.token_from.token.name,
      from_canister_id: swapState.token_from.token.canister_id,
      to: swapState.token_to.token.name,
      amount: Number(swapState.send_amount_input),
      enabled: !!unauthenticatedAgent && !!Number(swapState.send_amount_input),
    }
  );

  const onChangeToken = (selectedToken: Token) => {
    dispatchSwapState({ type: "SET_TOKEN_TO", value: selectedToken });
  };

  return (
    <div className="flex flex-col gap-4 p-4 border border-border rounded-xl bg-surface-secondary">
      <div className="text-copper font-semibold">Receive</div>
      <div className="flex justify-between items-start">
        <div>
          <div className="text-2xl">
            {swapAmount.isSuccess && balance.isSuccess ? (
              <div>
                {Number(swapAmount.data.receive_amount) /
                  10 ** balance.data.decimals}
              </div>
            ) : (
              <div className="animate-pulse">0</div>
            )}
          </div>
          <div className="text-xs">
            {swapAmount.isSuccess && balance.isSuccess ? (
              <div>
                ≈$
                <NumberToLocaleString
                  value={
                    (Number(swapAmount.data.receive_amount) /
                      10 ** balance.data.decimals) *
                    balance.data.price_usd
                  }
                />
              </div>
            ) : (
              <div className="animate-pulse">≈$0</div>
            )}
          </div>
        </div>
        <div className="">
          <ListboxToken
            value={swapState.token_to.token}
            options={TOKENS}
            optionsDisabled={TOKENS.filter(
              (t) => t.id === swapState.token_from.token.id
            )}
            onChange={onChangeToken}
          ></ListboxToken>
        </div>
      </div>
      <BalanceAvailable token={swapState.token_to.token} balance={balance} />
    </div>
  );
};

export default ReceiveForm;
