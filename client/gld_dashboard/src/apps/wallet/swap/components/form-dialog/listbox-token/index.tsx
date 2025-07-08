import { Listbox, ListboxButton, ListboxOptions } from "@headlessui/react";
import clsx from "clsx";
import { Logo } from "@components/index";
import { Token } from "@shared/utils/tokens";
import Icon from "@shared/ui/icons";
import ListboxOptionToken from "@wallet/swap/components/form-dialog/listbox-token/ListboxOptionToken";

const ListboxToken = ({
  value,
  onChange,
  options,
  optionsDisabled = [],
  className,
}: {
  value: Token;
  onChange: (selectedToken: Token) => void;
  options: Token[];
  optionsDisabled?: Token[];
  className?: string;
}) => {
  return (
    <div className={className}>
      <Listbox value={value} onChange={onChange}>
        <ListboxButton
          className={clsx(
            "rounded-full shadow-md py-2 px-4 w-full bg-surface-primary",
            "cursor-pointer"
          )}
        >
          <div className="flex justify-between items-center">
            <div className="flex items-center gap-2">
              <Logo name={value.id} className="w-6 h-6" />
              <div>{value.name}</div>
            </div>
            <Icon.Chevron width={18} height={18} className="ml-4" />
          </div>
        </ListboxButton>
        <ListboxOptions
          anchor="left"
          transition
          className={clsx(
            "w-md z-50 mt-1",
            "rounded-xl border border-border bg-surface-primary shadow-lg",
            "text-sm/6 focus:outline-none",
            "transition duration-100 ease-out",
            "data-[closed]:scale-95 data-[closed]:opacity-0"
          )}
        >
          {options.map((token) => (
            <ListboxOptionToken
              value={token}
              key={token.name}
              disabled={optionsDisabled.includes(token)}
            />
          ))}
        </ListboxOptions>
      </Listbox>
    </div>
  );
};

export default ListboxToken;
