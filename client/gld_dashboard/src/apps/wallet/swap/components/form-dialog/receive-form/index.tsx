import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { Token, TOKENS } from "@shared/utils/tokens";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import ListboxToken from "@wallet/swap/components/form-dialog/listbox-token";
import BalanceAvailable from "@shared/components/BalanceAvailable";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ReceiveForm = () => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const balance = useFetchLedgerBalance(
    swapState.receive_token.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.receive_token.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
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
            {swapState.receive_amount !== undefined && balance.isSuccess ? (
              <NumberToLocaleString
                value={
                  Number(swapState.receive_amount) / 10 ** balance.data.decimals
                }
                decimals={5}
              />
            ) : (
              <div className="animate-pulse">0</div>
            )}
          </div>
          <div className="text-xs">
            {swapState.receive_amount !== undefined && balance.isSuccess ? (
              <div>
                ≈$
                <NumberToLocaleString
                  value={
                    (Number(swapState.receive_amount) /
                      10 ** balance.data.decimals) *
                    balance.data.price_usd
                  }
                  decimals={5}
                />
              </div>
            ) : (
              <div className="animate-pulse">≈$0</div>
            )}
          </div>
        </div>
        <div className="">
          <ListboxToken
            value={swapState.receive_token.token}
            options={TOKENS}
            optionsDisabled={TOKENS.filter(
              (t) => t.id === swapState.send_token.token.id
            )}
            onChange={onChangeToken}
          ></ListboxToken>
        </div>
      </div>
      <div className="text-sm text-content/80">
        <BalanceAvailable
          token={swapState.receive_token.token.name}
          balance={balance.data?.balance}
        />
      </div>
    </div>
  );
};

export default ReceiveForm;
