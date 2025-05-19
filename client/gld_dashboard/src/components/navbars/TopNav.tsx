import { useState } from "react";
import { Link } from "react-router-dom";
import { Dialog } from "@headlessui/react";
import { XMarkIcon, Bars3Icon } from "@heroicons/react/20/solid";

import { useAuth } from "@auth/index";

import DropdownUserMenu from "@components/dropdowns/UserMenu";
import { Button } from "@components/index";
import { Logo } from "@components/logos";

import navItems from "./navItems.utils";
import clsx from "clsx";

const TopNav = ({ className }: { className?: string }) => {
  const { isConnected, connect } = useAuth();
  const [showMenu, setShowMenu] = useState(false);

  const handleOnHideMenu = () => setShowMenu(false);

  return (
    <nav className={className}>
      <div className="flex justify-between items-center">
        {/* Brand */}
        <div className="flex-shrink-0">
          <Link to="/" className="flex items-center space-x-2">
            <Logo name="gold_dao" />
            <span className="self-center text-xl font-semibold whitespace-nowrap hidden sm:block">
              Gold DAO
            </span>
          </Link>
        </div>

        {/* Menu */}
        <div className="flex justify-self-end items-center">
          {!isConnected && (
            <Button
              className={clsx(
                "px-4 py-2 rounded-xl w-full",
                "bg-secondary text-white"
              )}
              onClick={connect}
            >
              Connect Wallet
            </Button>
          )}

          {isConnected && (
            <div className="flex items-center gap-2 bg-surface-primary border border-border rounded-lg">
              <DropdownUserMenu />
            </div>
          )}
          <div className="xl:hidden">
            <button
              onClick={() => setShowMenu(!showMenu)}
              type="button"
              className="inline-flex items-center justify-center p-2 rounded-full hover:bg-surface-secondary focus:outline-none"
            >
              <span className="sr-only">Open main menu</span>
              <Bars3Icon className="h-6 w-6" />
            </button>
          </div>
        </div>
      </div>

      {/* Menu Mobile */}
      {showMenu && (
        <div className="fixed z-50 inset-0 overflow-hidden">
          <Dialog
            as="div"
            className="fixed z-50 inset-0 overflow-hidden"
            open={showMenu}
            onClose={handleOnHideMenu}
          >
            <div
              className="absolute z-50 inset-0 overflow-hidden"
              onClick={() => setShowMenu(false)}
            >
              <div className="fixed w-full inset-0 bg-black/50" />
              <div className="fixed inset-x-0 top-0 w-full flex">
                <div className="bg-background w-full px-8 py-5">
                  <div className="flex flex-col items-center px-2 pt-2 pb-3 space-y-1 sm:px-3">
                    <div className="flex items-center justify-between w-full mb-4">
                      <Link to="/" className="flex items-center pr-4">
                        <Logo name="gold_dao" />
                        <span className="self-center text-xl font-semibold whitespace-nowrap ml-2">
                          Gold DAO
                        </span>
                      </Link>
                      <button
                        onClick={() => setShowMenu(!showMenu)}
                        type="button"
                        className="inline-flex items-center justify-center p-2 rounded-full hover:bg-surface-secondary focus:outline-none"
                      >
                        <span className="sr-only">Open main menu</span>
                        <XMarkIcon className="h-6 w-6" />
                      </button>
                    </div>

                    {navItems.map(({ title, url }, i) => (
                      <Link
                        onClick={handleOnHideMenu}
                        to={url}
                        className="font-semibold text-content/60 hover:text-content px-3 py-2 rounded-md"
                        key={i}
                      >
                        {title}
                      </Link>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </Dialog>
        </div>
      )}
    </nav>
  );
};

export default TopNav;
