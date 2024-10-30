import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/16/solid";

import { useLedgerUserBalance } from "@hooks/ledger";

import { Skeleton } from "@components/ui";
import { LogoGLDT } from "@components/shared/logos";

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
                    <LogoGLDT className="flex-none w-6 h-6" />
                    <div className="truncate hidden sm:block">
                      <div className="truncate font-semibold text-sm">
                        {balanceGLDT.string} GLDT
                      </div>
                    </div>
                  </div>
                  <ChevronDownIcon className="size-4 fill-content/60" />
                </MenuButton>

                <MenuItems
                  transition
                  anchor="bottom end"
                  className="max-w-fit z-50 border border-border origin-top-right rounded-xl bg-surface py-1 pl-1 pr-3 mt-1 text-sm/6 transition duration-100 ease-out [--anchor-gap:var(--spacing-1)] focus:outline-none data-[closed]:scale-95 data-[closed]:opacity-0"
                >
                  {isSuccessBalanceGLDT && balanceGLDT && (
                    <MenuItem>
                      <div className="flex items-center gap-2 pl-2 py-2">
                        <LogoGLDT className="flex-none w-5 h-5" />
                        <div className="font-semibold text-sm">
                          {balanceGLDT.string}
                        </div>
                        <div className="font-semibold text-sm">GLDT</div>
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
                        <div className="font-semibold text-sm">OGY</div>
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
