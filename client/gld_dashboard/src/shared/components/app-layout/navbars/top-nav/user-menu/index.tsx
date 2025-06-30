import { Link } from "react-router-dom";
import clsx from "clsx";
import { Menu, MenuButton, MenuItem, MenuItems } from "@headlessui/react";
import { useAuth } from "@auth/index";
import Address from "@components/strings/Address";
import Icon from "@shared/ui/icons";
import { useCopyToClipboard } from "@shared/hooks/useCopyToClipboard";

const DropdownUserMenu = () => {
  const { principalId, disconnect } = useAuth();
  const { copyToClipboard } = useCopyToClipboard();

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
            <div className="relative">
              <MenuButton className="inline-flex items-center gap-2 rounded-lg bg-surface-primary py-1.5 px-3 text-sm/6 font-semibold cursor-pointer">
                <div className="flex items-center gap-2">
                  <Icon.User width={16} height={16} />
                  <div className="hidden xl:block">
                    <Address
                      size="lg"
                      enableTooltip={false}
                      enableCopyToClipboard={false}
                    >
                      {principalId}
                    </Address>
                  </div>
                  <div className="block xl:hidden">
                    <Address
                      size="sm"
                      enableTooltip={false}
                      enableCopyToClipboard={false}
                    >
                      {principalId}
                    </Address>
                  </div>
                </div>
                <Icon.Chevron
                  width={16}
                  height={16}
                  className={clsx({ "rotate-180": open })}
                />
              </MenuButton>
            </div>

            <MenuItems
              transition
              anchor="bottom end"
              className="w-64 sm:w-[var(--button-width)] z-50 border border-surface-primary origin-top-right rounded-xl bg-surface-primary p-1 mt-1 text-sm/6 transition duration-100 ease-out [--anchor-gap:var(--spacing-1)] focus:outline-none data-[closed]:scale-95 data-[closed]:opacity-0 text-content"
            >
              <MenuItem>
                <button
                  onClick={() => copyToClipboard(principalId)}
                  className="cursor-pointer flex w-full items-center gap-2 py-1.5 px-3"
                  data-tooltip-content={principalId}
                  data-tooltip-id="tooltip"
                >
                  <Icon.Copy width={16} height={16} />
                  Copy address
                </button>
              </MenuItem>

              <div className="my-1 h-px border-t border-border" />

              <MenuItem>
                <Link
                  to="wallet"
                  className="flex w-full items-center gap-2 py-1.5 px-3"
                >
                  <Icon.Transfer width={16} height={16} />
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
                  <Icon.ExternalLink width={16} height={16} />
                  View in explorer
                </a>
              </MenuItem>

              <div className="my-1 h-px border-t border-border" />

              <MenuItem>
                <div
                  onClick={handleDisconnect}
                  className="flex w-full items-center gap-2 rounded-lg py-1.5 px-3 cursor-pointer"
                >
                  <Icon.Logout width={16} height={16} />
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
