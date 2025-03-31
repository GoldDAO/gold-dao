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
import E8sToLocaleString from "@components/numbers/E8sToLocaleString";

import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";

import TOKENS_LIST, { Token } from "./tokensList.utils";

const ListboxTokenOption = ({ id, name, label, canisterId }: Token) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const { data, isSuccess } = useFetchUserBalance(
    canisterId,
    unauthenticatedAgent,
    {
      ledger: id,
      owner: principalId,
      enabled: !!unauthenticatedAgent && !!isConnected,
    }
  );

  return (
    <div className="m-2 font-semibold text-sm cursor-default hover:bg-surface-secondary hover:rounded-lg">
      <div className="flex justify-between items-center p-4">
        <div className="flex items-center gap-2">
          <Logo name={id} className="h-10 w-10" />
          <div>
            <div>{name}</div>
            <div className="text-content/60">{label}</div>
          </div>
        </div>
        <div className="">
          {isSuccess ? (
            <E8sToLocaleString value={data} />
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

  const { data, isSuccess } = useFetchUserBalance(
    selected.canisterId,
    unauthenticatedAgent,
    {
      ledger: selected.id,
      owner: principalId,
      enabled: !!unauthenticatedAgent && !!isConnected,
    }
  );

  const handleChange = (token: Token) => {
    setSelected(token);
    handleOnChange(token);
  };

  return (
    <div className={className}>
      <Listbox value={selected} onChange={handleChange}>
        <ListboxButton
          className={clsx(
            "w-[300px] rounded-full border border-border py-2 px-4",
            "text-sm/6"
          )}
          disabled={!isSuccess}
        >
          <div className="cursor-default">
            {isSuccess ? (
              <div className="flex justify-between items-center">
                <div className="flex items-center gap-2">
                  <Logo name={selected.id} className="w-6 h-6" />
                  Balance: <E8sToLocaleString value={data} /> {selected.name}
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
