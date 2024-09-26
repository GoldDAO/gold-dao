import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/16/solid";

import { useLedgerUserBalance } from "@hooks/ledger";

import { Skeleton } from "@components/ui";

const DropdownBalance = () => {
  const { data: balanceOGY, isSuccess: isSuccessBalanceOGY } =
    useLedgerUserBalance({ ledger: "OGY" });
  const { data: balanceGLDT, isSuccess: isSuccessBalanceGLDT } =
    useLedgerUserBalance({ ledger: "GLDT" });

  return (
    <div className="text-center">
      {isSuccessBalanceGLDT && balanceGLDT ? (
        <Menu>
          {({ open }) => (
            <>
              <div
                className={`fixed inset-0 bg-black transition-opacity ${
                  open ? "opacity-20" : "opacity-0 pointer-events-none"
                }`}
              />

              <>
                <MenuButton className="inline-flex items-center gap-2 rounded-lg bg-surface py-1.5 px-3 text-sm/6 font-semibold">
                  <div className="max-w-48 flex items-center gap-2">
                    <img className="flex-none h-6" src={`/gldt_logo.svg`} />
                    <div className="truncate hidden sm:block">
                      <div className="truncate font-semibold text-sm">
                        {balanceGLDT.string} GLDT
                      </div>
                    </div>
                  </div>
                  <ChevronDownIcon className="size-4 fill-content/60" />
                </MenuButton>
                {/* <MenuButton className="inline-flex items-center gap-2 rounded-lg bg-surface py-1.5 px-3 text-sm/6 font-semibold">
                  <div className="max-w-48 flex items-center gap-2 px-2">
                    <img className="flex-none h-6" src={`/gldt_logo.svg`} />
                    <div className="font-semibold text-sm">
                      {balanceGLDT.string}
                    </div>
                    <div className="font-semibold text-sm hidden md:block">
                      GLDT
                    </div>
                  </div>
                  <ChevronDownIcon className="size-4 fill-content/60" />
                </MenuButton> */}

                <MenuItems
                  transition
                  anchor="bottom end"
                  className="w-48 sm:w-[var(--button-width)] z-50 border border-border origin-top-right rounded-xl bg-surface p-1 mt-1 text-sm/6 transition duration-100 ease-out [--anchor-gap:var(--spacing-1)] focus:outline-none data-[closed]:scale-95 data-[closed]:opacity-0"
                >
                  {isSuccessBalanceGLDT && balanceGLDT && (
                    <MenuItem>
                      <div className="flex items-center gap-2 pl-2 py-2">
                        <img className="flex-none h-5" src={`/gldt_logo.svg`} />
                        <div className="font-semibold text-sm">
                          {balanceGLDT.string}
                        </div>
                        <div className="font-semibold text-sm hidden sm:block">
                          GLDT
                        </div>
                      </div>
                    </MenuItem>
                  )}
                  {isSuccessBalanceOGY && balanceOGY && (
                    <MenuItem>
                      <div className="flex items-center gap-2 pl-2 py-2">
                        <img className="flex-none h-5" src={`/ogy_logo.svg`} />
                        <div className="font-semibold text-sm">
                          {balanceOGY.string}
                        </div>
                        <div className="font-semibold text-sm hidden sm:block">
                          OGY
                        </div>
                      </div>
                    </MenuItem>
                  )}
                </MenuItems>
              </>
            </>
          )}
        </Menu>
      ) : (
        <Skeleton className="w-32" />
      )}
    </div>
  );
};

export default DropdownBalance;
