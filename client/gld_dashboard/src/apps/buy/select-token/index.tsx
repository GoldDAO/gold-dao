import { useState } from "react";
import {
  Listbox,
  ListboxButton,
  ListboxOption,
  ListboxOptions,
} from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import { Token } from "@buy/shared/utils";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ListboxTokenOption = ({ id, name, label, canisterId }: Token) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const balance = useFetchLedgerBalance(canisterId, unauthenticatedAgent, {
    ledger: name,
    owner: principalId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const renderBalance = () => {
    if (balance.isSuccess) {
      return <NumberToLocaleString value={balance.data.balance} />;
    } else {
      return <div>Loading...</div>;
    }
  };

  return (
    <div className="m-2 font-semibold text-sm cursor-pointer hover:bg-surface-secondary hover:rounded-lg">
      <div className="flex justify-between items-center p-4">
        <div className="flex items-center gap-2">
          <Logo name={id} className="h-10 w-10" />
          <div>
            <div>{name}</div>
            <div className="text-content/60">{label}</div>
          </div>
        </div>
        <div>{isConnected && renderBalance()}</div>
      </div>
    </div>
  );
};

const SelectToken = ({
  value,
  tokens,
  handleOnChange,
  className,
}: {
  value: Token;
  tokens: Token[];
  handleOnChange: (selectedToken: Token) => void;
  className?: string;
}) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();
  const [selected, setSelected] = useState(value);

  const balance = useFetchLedgerBalance(
    selected.canisterId,
    unauthenticatedAgent,
    {
      ledger: selected.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const handleChange = (token: Token) => {
    setSelected(token);
    handleOnChange(token);
  };

  const renderBalance = () => {
    if (balance.isSuccess) {
      return (
        <>
          Balance: <NumberToLocaleString value={balance.data.balance} />{" "}
          {selected.name}
        </>
      );
    } else {
      return <div>Fetching your {selected.name} balance...</div>;
    }
  };

  return (
    <div className={className}>
      <Listbox value={selected} onChange={handleChange}>
        <ListboxButton
          className={clsx(
            "rounded-md border border-border p-4 w-full",
            "text-sm/6 cursor-pointer"
          )}
        >
          <div className="flex justify-between items-center">
            <div className="flex items-center gap-2">
              <Logo name={selected.id} className="w-6 h-6" />
              {isConnected ? renderBalance() : <div>{selected.name}</div>}
            </div>
            <ChevronDownIcon className="ml-4 h-6 w-6" />
          </div>
        </ListboxButton>
        <ListboxOptions
          anchor="bottom"
          transition
          className={clsx(
            "w-[var(--button-width)] z-50 border border-border rounded-xl bg-surface-primary mt-1 text-sm/6 focus:outline-none shadow-lg",
            "transition duration-100 ease-out data-[closed]:scale-95 data-[closed]:opacity-0"
          )}
        >
          {tokens.map((token) => (
            <ListboxOption key={token.name} value={token}>
              <ListboxTokenOption
                id={token.id}
                name={token.name}
                label={token.label}
                canisterId={token.canisterId}
              />
            </ListboxOption>
          ))}
        </ListboxOptions>
      </Listbox>
    </div>
  );
};

export default SelectToken;
