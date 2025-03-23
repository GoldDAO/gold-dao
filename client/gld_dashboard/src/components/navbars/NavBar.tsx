import { Link } from "react-router-dom";

import { useAuth } from "@auth/index";

import { useState, Fragment } from "react";
import { useLocation } from "react-router-dom";
import { Transition, TransitionChild, Dialog } from "@headlessui/react";
import { XMarkIcon, Bars3Icon } from "@heroicons/react/20/solid";
import {
  Button,
  DropdownUserMenu,
  DropdownUserBalance,
} from "@components/index";
import { Logo } from "@components/logos";

export const NavBar = () => {
  const { isConnected, connect } = useAuth();
  const [showMenu, setShowMenu] = useState(false);
  const location = useLocation();
  const active = location.pathname;

  const navItems: { title: string; url: string }[] = [
    { title: "Dashboard", url: "/dashboard" },
    { title: "SNS", url: "/dashboard/sns" },
    { title: "Stake", url: "/stake" },
    { title: "Account", url: "/dashboard/account" },
  ];

  const handleOnHideMenu = () => setShowMenu(false);

  return (
    <nav className="sticky top-0 py-5 z-40 px-4 sm:px-6 bg-background">
      <div className="grid grid-cols-2 md:grid-cols-3 items-center h-10">
        <div className="flex-shrink-0">
          <Link to="/" className="flex items-center space-x-2">
            <Logo name="gold_dao" />
            <span className="self-center text-xl font-semibold whitespace-nowrap hidden sm:block">
              Gold DAO Dashboard
            </span>
          </Link>
        </div>
        <div className="hidden md:block justify-self-center">
          <div className="flex items-center justify-end space-x-12">
            {navItems.map(({ title, url }, i) => (
              <Link
                to={url}
                className={`text-content hover:font-semibold ${
                  active === url ? "font-semibold" : ""
                }`}
                key={i}
              >
                {title}
              </Link>
            ))}
          </div>
        </div>
        <div className="flex justify-self-end items-center">
          {!isConnected && <Button onClick={connect}>Connect</Button>}
          {isConnected && (
            <div className="flex items-center gap-2 bg-surface-primary p-[2px] border border-border rounded-lg">
              <DropdownUserBalance />
              <DropdownUserMenu />
            </div>
          )}

          <div className="md:hidden">
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
      {/* Mobile menu */}
      <Transition show={showMenu} as={Fragment}>
        <div className="fixed z-50 inset-0 overflow-hidden">
          <Dialog
            as={Fragment}
            static
            open={showMenu}
            onClose={handleOnHideMenu}
          >
            <div
              className="absolute z-50 inset-0 overflow-hidden"
              onClick={() => setShowMenu(false)}
            >
              <TransitionChild
                as={Fragment}
                enter="ease-in-out duration-500"
                enterFrom="opacity-0"
                enterTo="opacity-100"
                leave="ease-in-out duration-500"
                leaveFrom="opacity-100"
                leaveTo="opacity-0"
              >
                <div className="fixed w-full inset-0 bg-black bg-opacity-50 transition-opacity" />
              </TransitionChild>
              <div className="fixed inset-x-0 top-0 w-full flex">
                <TransitionChild
                  as={Fragment}
                  enter="transform transition ease-in-out duration-500 sm:duration-700"
                  enterFrom="-translate-y-full"
                  enterTo="translate-y-0"
                  leave="transform transition ease-in-out duration-500 sm:duration-700"
                  leaveFrom="translate-y-0"
                  leaveTo="-translate-y-full"
                >
                  <div className="bg-background w-full px-8 py-5">
                    <div className="flex flex-col items-center px-2 pt-2 pb-3 space-y-1 sm:px-3">
                      <div className="flex items-center justify-between w-full mb-4">
                        <Link to="/" className="flex items-center pr-4">
                          <Logo name="gold_dao" />
                          <span className="self-center text-xl font-semibold whitespace-nowrap ml-2">
                            Gold DAO Dashboard
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
                </TransitionChild>
              </div>
            </div>
          </Dialog>
        </div>
      </Transition>
    </nav>
  );
};
