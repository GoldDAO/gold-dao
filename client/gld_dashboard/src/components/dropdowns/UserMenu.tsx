import { useState, useCallback } from "react";
import { toast } from "react-hot-toast";
import { Link } from "react-router-dom";
import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/react";
import { CopyToClipboard } from "react-copy-to-clipboard";
import {
  ChevronDownIcon,
  UserIcon,
  ArrowsRightLeftIcon,
  ArrowUpTrayIcon,
  ClipboardDocumentIcon,
  EllipsisHorizontalIcon,
} from "@heroicons/react/24/outline";

import { useAuth } from "@auth/index";

import { Tile } from "@components/index";

const DropdownUserMenu = () => {
  const { principalId, disconnect } = useAuth();
  const [, setCopied] = useState(false);

  const handleDisconnect = () => {
    disconnect();
  };

  const onCopy = useCallback(() => {
    setCopied(true);
    toast.success("That's copied!");
  }, []);

  const handleClick = (e: React.MouseEvent<HTMLElement>) => {
    e.preventDefault();
    e.stopPropagation();
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
            <div className="relative">
              <MenuButton className="inline-flex items-center gap-2 rounded-lg bg-surface-primary py-1.5 px-3 text-sm/6 font-semibold">
                <div className="max-w-48 flex items-center gap-2">
                  <Tile className="rounded-full h-6 w-6 bg-accent/60">
                    <UserIcon className="p-1 text-primary" />
                  </Tile>
                  <div className="truncate hidden sm:block text-primary">
                    {principalId}
                  </div>
                </div>
                <ChevronDownIcon className="size-4 fill-content/60" />
              </MenuButton>
            </div>

            <MenuItems
              transition
              anchor="bottom end"
              className="w-64 sm:w-[var(--button-width)] z-50 border border-surface-primary origin-top-right rounded-xl bg-surface-primary p-1 mt-1 text-sm/6 transition duration-100 ease-out [--anchor-gap:var(--spacing-1)] focus:outline-none data-[closed]:scale-95 data-[closed]:opacity-0 text-content"
            >
              <MenuItem>
                <div className="cursor-pointer">
                  <CopyToClipboard onCopy={onCopy} text={principalId}>
                    <div
                      onClick={handleClick}
                      className="group flex w-full items-center gap-2 py-1.5 px-3"
                    >
                      <ClipboardDocumentIcon className="size-4" />
                      Copy address
                    </div>
                  </CopyToClipboard>
                </div>
              </MenuItem>

              <div className="my-1 h-px border-t border-border" />

              <MenuItem>
                <Link
                  to="wallet"
                  className="group flex w-full items-center gap-2 py-1.5 px-3"
                >
                  <ArrowsRightLeftIcon className="size-4" />
                  Transfer
                </Link>
              </MenuItem>

              <div className="my-1 h-px border-t border-border" />

              <MenuItem>
                <a
                  href={`https://www.icexplorer.io/address/details/${principalId}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className={`group flex w-full items-center gap-2 py-1.5 px-3`}
                >
                  <EllipsisHorizontalIcon className="size-4" />
                  View in explorer
                </a>
              </MenuItem>

              <div className="my-1 h-px border-t border-border" />

              <MenuItem>
                <div
                  onClick={handleDisconnect}
                  className="group flex w-full items-center gap-2 rounded-lg py-1.5 px-3 cursor-pointer"
                >
                  <ArrowUpTrayIcon className="size-4 rotate-90" />
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

export default DropdownUserMenu;
