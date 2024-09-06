import { ReactNode } from "react";
import {
  Listbox,
  ListboxButton,
  ListboxOptions,
  ListboxOption,
} from "@headlessui/react";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/react/20/solid";

const Select = ({
  options,
  value,
  handleOnChange,
  className,
  placeholder,
}: {
  options: Array<{ value: string | number }>;
  value: string | number;
  handleOnChange: (v: string | number) => void;
  className?: string;
  placeholder?: ReactNode;
}) => {
  const displayValue = value === "" ? placeholder : value;

  return (
    <div className={`${className}`}>
      <Listbox value={value} onChange={handleOnChange}>
        <ListboxButton className="relative w-full cursor-default rounded-xl bg-surface-2 py-2 pl-3 pr-10 text-left border border-border focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white/75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300 sm:text-sm">
          <span className="block truncate">{displayValue}</span>
          <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
            <ChevronUpDownIcon className="h-5 w-5" aria-hidden="true" />
          </span>
        </ListboxButton>

        <ListboxOptions
          anchor="bottom"
          className="max-h-60 w-auto overflow-auto rounded-xl bg-surface-2 z-50 py-1 text-base border border-border focus:outline-none sm:text-sm origin-top transition duration-200 ease-out data-[closed]:scale-95 data-[closed]:opacity-0"
        >
          {options.map((option) => (
            <ListboxOption
              key={option.value}
              className={({ focus }) =>
                `relative cursor-default select-none py-2 pl-10 pr-4 ${
                  focus ? "bg-accent/10 text-accent" : ""
                }`
              }
              value={option.value}
            >
              {({ selected }) => (
                <>
                  <span
                    className={`block truncate ${
                      selected ? "font-medium" : "font-normal"
                    }`}
                  >
                    {option.value}
                  </span>
                  {selected ? (
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-accent">
                      <CheckIcon className="h-5 w-5" aria-hidden="true" />
                    </span>
                  ) : (
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-content/10">
                      <CheckIcon className="h-5 w-5" aria-hidden="true" />
                    </span>
                  )}
                </>
              )}
            </ListboxOption>
          ))}
        </ListboxOptions>
      </Listbox>
    </div>
  );
};

export default Select;
