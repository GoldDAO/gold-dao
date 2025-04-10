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
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import TOKENS_LIST, { Token } from "./tokensList.utils";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

const ListboxTokenOption = ({ id, name, label, canisterId }: Token) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const balance = useFetchUserBalance(canisterId, unauthenticatedAgent, {
    ledger: id,
    owner: principalId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const decimals = useFetchDecimals(canisterId, unauthenticatedAgent, {
    ledger: id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

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
        <div className="">
          {balance.isSuccess && decimals.isSuccess ? (
            <TokenValueToLocaleString
              value={balance.data}
              tokenDecimals={decimals.data}
            />
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    </div>
  );
};

const SelectBuyMethod = ({
  value,
  handleOnChange,
  className,
}: {
  value: Token;
  handleOnChange: (selectedToken: Token) => void;
  className?: string;
}) => {
  const [selected, setSelected] = useState(value);

  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const balance = useFetchUserBalance(
    selected.canisterId,
    unauthenticatedAgent,
    {
      ledger: selected.id,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const decimals = useFetchDecimals(selected.canisterId, unauthenticatedAgent, {
    ledger: selected.id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const handleChange = (token: Token) => {
    setSelected(token);
    handleOnChange(token);
  };

  return (
    <div className={className}>
      <Listbox value={selected} onChange={handleChange}>
        <ListboxButton
          className={clsx(
            "min-w-[300px] rounded-md border border-border p-4",
            "text-sm/6"
          )}
          disabled={!balance.isSuccess || !decimals.isSuccess}
        >
          <div className="cursor-pointer">
            {balance.isSuccess && decimals.isSuccess ? (
              <div className="flex justify-between items-center">
                <div className="flex items-center gap-2">
                  <Logo name={selected.id} className="w-6 h-6" />
                  Balance:{" "}
                  <TokenValueToLocaleString
                    value={balance.data}
                    tokenDecimals={decimals.data}
                  />{" "}
                  {selected.name}
                </div>
                <ChevronDownIcon className="h-6 w-6" />
              </div>
            ) : (
              <div className="flex">Loading...</div>
            )}
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
          {TOKENS_LIST.map((token) => (
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

export default SelectBuyMethod;
