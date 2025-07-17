import { ListboxOption } from "@headlessui/react";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import { Token } from "@shared/utils/tokens";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ListboxOptionToken = ({
  value,
  disabled = false,
}: {
  value: Token;
  disabled?: boolean;
}) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const balance = useFetchLedgerBalance(
    value.canister_id,
    unauthenticatedAgent,
    {
      ledger: value.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const renderBalance = () => {
    if (balance.isSuccess) {
      return <NumberToLocaleString value={balance.data.balance} />;
    } else {
      return <div>Loading...</div>;
    }
  };

  return (
    <ListboxOption value={value} disabled={disabled}>
      <div
        className={`m-2 font-semibold text-sm ${
          disabled ? "cursor-not-allowed opacity-50" : "cursor-pointer"
        } hover:bg-surface-secondary hover:rounded-lg`}
      >
        <div className="flex justify-between items-center p-4">
          <div className="flex items-center gap-2">
            <Logo name={value.id} className="h-10 w-10" />
            <div>
              <div>{value.display_name}</div>
              <div className="text-content/60">{value.label}</div>
            </div>
          </div>
          <div>{isConnected && renderBalance()}</div>
        </div>
      </div>
    </ListboxOption>
  );
};

export default ListboxOptionToken;
