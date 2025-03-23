import { Link } from "react-router-dom";
import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/react";
import {
  ChevronDownIcon,
  UserIcon,
  ArrowsRightLeftIcon,
  ArrowUpTrayIcon,
} from "@heroicons/react/16/solid";

import { useAuth } from "@auth/index";

import { Tile } from "@components/index";
import CopyToClipboard from "@components/buttons/CopyToClipboard";

export const DropdownUserMenu = () => {
  const { principalId, disconnect } = useAuth();

  const handleDisconnect = () => {
    disconnect();
  };

  return (
    <div className="text-center">
      <Menu>
        {({ open }) => (
          <>
            <div
              className={`fixed inset-0 bg-black transition-opacity ${
                open ? "opacity-20" : "opacity-0 pointer-events-none"
              }`}
            />
            <MenuButton className="inline-flex items-center gap-2 rounded-lg bg-surface-primary py-1.5 px-3 text-sm/6 font-semibold">
              <div className="max-w-48 flex items-center gap-2">
                <Tile className="rounded-full h-6 w-6 bg-accent/60">
                  <UserIcon className="p-1 text-content" />
                </Tile>
                <div className="truncate hidden sm:block">{principalId}</div>
              </div>
              <ChevronDownIcon className="size-4 fill-content/60" />
            </MenuButton>

            <MenuItems
              transition
              anchor="bottom end"
              className="w-64 sm:w-[var(--button-width)] z-50 border border-border origin-top-right rounded-xl bg-surface-primary p-1 mt-1 text-sm/6 transition duration-100 ease-out [--anchor-gap:var(--spacing-1)] focus:outline-none data-[closed]:scale-95 data-[closed]:opacity-0"
            >
              <MenuItem>
                <div className="bg-accent/5 border border-gold/20 rounded-xl m-3 py-2 px-1">
                  <div className="flex justify-center">
                    <Tile className="rounded-full h-8 w-8 bg-accent/60 mb-2">
                      <UserIcon className="p-1 text-content" />
                    </Tile>
                  </div>
                  <div className="flex items-center truncate pr-2">
                    <div className="flex ml-4 items-center truncate text-sm">
                      <div
                        className="truncate"
                        data-tooltip-id="tooltip"
                        data-tooltip-content={principalId}
                      >
                        {principalId}
                      </div>
                      <CopyToClipboard value={principalId} />
                    </div>
                  </div>
                </div>
              </MenuItem>

              <MenuItem>
                <Link
                  to="/account"
                  className="group flex w-full items-center gap-2 rounded-lg py-1.5 px-3 data-[focus]:bg-white/10"
                >
                  <UserIcon className="size-4" />
                  My account
                </Link>
              </MenuItem>

              <MenuItem>
                <Link
                  to="swap?view=1"
                  className="group flex w-full items-center gap-2 rounded-lg py-1.5 px-3 data-[focus]:bg-white/10"
                >
                  <ArrowsRightLeftIcon className="size-4" />
                  Transfer
                </Link>
              </MenuItem>

              <div className="my-1 h-px border-t border-border" />

              <MenuItem>
                <div
                  onClick={handleDisconnect}
                  className="group flex w-full items-center gap-2 rounded-lg py-1.5 px-3 data-[focus]:bg-white/10 cursor-pointer"
                >
                  <ArrowUpTrayIcon className="size-4" />
                  Log out
                </div>
              </MenuItem>
            </MenuItems>
          </>
        )}
      </Menu>
    </div>
  );
};
