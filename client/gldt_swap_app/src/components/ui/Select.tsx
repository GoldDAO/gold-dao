import { ReactNode } from "react";
import {
  Listbox,
  ListboxButton,
  ListboxOptions,
  ListboxOption,
} from "@headlessui/react";
import { ChevronUpDownIcon } from "@heroicons/react/20/solid";

const Select = ({
  options,
  value,
  handleOnChange,
  className,
  // placeholder,
}: {
  options: Array<{ value: string | number; icon: string; label: string }>;
  value: string | number;
  handleOnChange: (v: string | number) => void;
  className?: string;
  placeholder?: ReactNode;
}) => {
  const displayValue = options.find((e) => e.value === value) as {
    value: string | number;
    icon: string;
    label: string;
  };

  return (
    <div className={`${className}`}>
      <Listbox value={value} onChange={handleOnChange}>
        <div className="relative mt-1">
          <ListboxButton className="relative w-full cursor-pointer rounded-full bg-surface py-2 pl-4 pr-11 text-left border border-border focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white/75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300 sm:text-sm">
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <img src={displayValue.icon} className="w-4 h-4 mr-2" />
                <div className={`truncate mr-3`}>{displayValue.value}</div>
              </div>
              <div className="bg-surface-2 rounded-sm px-2">
                {displayValue.label}
              </div>
            </div>
            <span className="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2">
              <ChevronUpDownIcon className="h-5 w-5" aria-hidden="true" />
            </span>
          </ListboxButton>

          <ListboxOptions
            anchor="bottom"
            className="absolute mt-1 max-h-60 w-auto overflow-auto rounded-xl bg-surface py-1 text-base border border-border focus:outline-none sm:text-sm origin-top transition duration-200 ease-out data-[closed]:scale-95 data-[closed]:opacity-0"
          >
            {options.map((option) => (
              <ListboxOption
                key={option.value}
                className={({ focus }) =>
                  `relative cursor-pointer select-none py-2 px-4 ${
                    focus ? "bg-accent/10 text-accent" : ""
                  }`
                }
                value={option.value}
              >
                {({ selected }) => (
                  <div className="flex items-center justify-between">
                    <div className="flex items-center">
                      <img src={option.icon} className="w-4 h-4 mr-2" />
                      <div
                        className={`truncate mr-3 ${
                          selected ? "font-semibold" : "font-normal"
                        }`}
                      >
                        {option.value}
                      </div>
                    </div>
                    <div className="bg-surface-2 rounded-sm px-2">
                      {option.label}
                    </div>
                  </div>
                )}
              </ListboxOption>
            ))}
          </ListboxOptions>
        </div>
      </Listbox>
    </div>
  );
};

export default Select;
