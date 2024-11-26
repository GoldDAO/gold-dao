import {
  Listbox,
  ListboxButton,
  ListboxOptions,
  ListboxOption,
} from "@headlessui/react";
import clsx from "clsx";
import { ChevronUpDownIcon } from "@heroicons/react/20/solid";

export const SelectSubaccount = ({
  options,
  value,
  handleOnChange,
  className,
  disabled = false,
}: {
  options: string[];
  value: string;
  handleOnChange: (v: string) => void;
  className?: string;
  disabled?: boolean;
}) => {
  return (
    <div className={className}>
      <Listbox value={value} onChange={handleOnChange} disabled={disabled}>
        <ListboxButton
          className={clsx(
            `${disabled ? "cursor-not-allowed" : ""}`,
            "relative block w-full rounded-full bg-surface border border-border py-1.5 text-left text-sm/6 pl-4 pr-11",
            "focus:outline-none data-[focus]:outline-2 data-[focus]:-outline-offset-2 data-[focus]:outline-white/25"
          )}
        >
          <div className={`truncate`}>{value}</div>
          <ChevronUpDownIcon className="group pointer-events-none absolute top-2.5 right-2.5 size-4" />
        </ListboxButton>
        <ListboxOptions
          anchor="bottom"
          transition
          className={clsx(
            "mt-1 w-[var(--button-width)] !max-h-64 overflow-y-auto rounded-xl border border-border bg-surface p-1 [--anchor-gap:var(--spacing-1)] focus:outline-none",
            "transition duration-100 ease-in data-[leave]:data-[closed]:opacity-0"
          )}
        >
          {options.map((option) => (
            <ListboxOption
              key={option}
              value={option}
              className="group flex cursor-pointer items-center gap-2 rounded-lg py-2 px-3 select-none data-[focus]:bg-accent/10 data-[focus]:text-accent data-[selected]:bg-accent/10 data-[selected]:text-accent data-[selected]:font-semibold"
            >
              <div
                className="truncate text-sm/6"
                data-tooltip-id="tooltip"
                data-tooltip-content={option}
              >
                {option}
              </div>
            </ListboxOption>
          ))}
        </ListboxOptions>
      </Listbox>

      {/* <Listbox value={value} onChange={handleOnChange}>
        <div className="relative mt-1">
          <ListboxButton className="relative w-full cursor-pointer rounded-full bg-surface py-2 pl-4 pr-11 text-left border border-border focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white/75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300 sm:text-sm">
            <div className="flex items-center justify-between">
              <div className={`truncate mr-3`}>{value}</div>
            </div>
            <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
              <ChevronUpDownIcon className="h-5 w-5" />
            </span>
          </ListboxButton>

          <ListboxOptions
            anchor="bottom"
            className={`w-[var(--button-width)] max-h-44 absolute mt-1 overflow-y-auto rounded-xl bg-surface py-1 text-base border border-border focus:outline-none sm:text-sm origin-top transition duration-200 ease-out data-[closed]:scale-95 data-[closed]:opacity-0`}
          >
            {options.map((option) => (
              <ListboxOption
                key={option}
                className={({ focus }) =>
                  `relative cursor-pointer select-none py-2 px-4 ${
                    focus ? "bg-accent/10 text-accent" : ""
                  }`
                }
                value={option}
              >
                {({ selected }) => (
                  <div className="flex items-center justify-between">
                    <div
                      className={`truncate mr-3 ${
                        selected ? "font-semibold" : "font-normal"
                      }`}
                    >
                      {option}
                    </div>
                  </div>
                )}
              </ListboxOption>
            ))}
          </ListboxOptions>
        </div>
      </Listbox> */}
    </div>
  );
};
