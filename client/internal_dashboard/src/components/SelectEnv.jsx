import { useState } from "react";
import {
  Listbox,
  ListboxButton,
  ListboxOption,
  ListboxOptions,
} from "@headlessui/react";
import clsx from "clsx";
import { ChevronDownIcon } from "@heroicons/react/20/solid";

const SelectEnv = ({ handleOnChange, className }) => {
  const [selected, setSelected] = useState({
    id: "staging",
    label: "Staging",
  });

  const envs = [
    {
      id: "staging",
      label: "Staging",
    },
    {
      id: "production",
      label: "Production",
    },
  ];

  const handleChange = (canister) => {
    setSelected(canister);
    handleOnChange(canister.id);
  };

  return (
    <div className={className}>
      <Listbox value={selected} onChange={handleChange}>
        <ListboxButton
          className={clsx(
            "bg-white/50 dark:bg-neutral-700 rounded-full px-6 py-4 w-38",
            "text-sm/6 cursor-pointer"
          )}
        >
          {({ open }) => (
            <div className="flex justify-between items-center">
              <div>{selected.label}</div>
              <ChevronDownIcon
                className={clsx(
                  "h-5 w-5 transition-transform duration-200",
                  open ? "rotate-180" : "rotate-0"
                )}
              />
            </div>
          )}
        </ListboxButton>
        <ListboxOptions
          anchor="bottom"
          transition
          className={clsx(
            "w-[var(--button-width)] z-50 rounded-xl shadow-md mt-1 text-sm/6 focus:outline-none",
            "transition duration-100 ease-out data-[closed]:scale-95 data-[closed]:opacity-0"
          )}
        >
          {envs.map((env) => (
            <ListboxOption key={env.id} value={env}>
              <div className="p-3 bg-white dark:bg-neutral-700 hover:bg-neutral-50 dark:hover:bg-neutral-600 cursor-pointer">
                {env.label}
              </div>
            </ListboxOption>
          ))}
        </ListboxOptions>
      </Listbox>
    </div>
  );
};

export default SelectEnv;
