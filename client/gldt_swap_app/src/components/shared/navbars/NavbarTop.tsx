import { useState, Fragment } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { Transition, TransitionChild, Dialog } from "@headlessui/react";
import { XMarkIcon, Bars3Icon } from "@heroicons/react/20/solid";

const NavbarTop = () => {
  const [showMenu, setShowMenu] = useState(false);

  const navItems: { title: string; anchor: string }[] = [
    { title: "GLDT", anchor: "/#gldt" },
    { title: "Ecosystem", anchor: "/#ecosystem" },
    { title: "Transparency", anchor: "/#transparency" },
    { title: "Explorer", anchor: "/explorer" },
    { title: "FAQ", anchor: "/#faq" },
  ];

  const location = useLocation();
  const navigate = useNavigate();

  // Scroll to anchor if present in hash
  const scrollToHash = (hash: string) => {
    if (!hash) return;
    const id = hash.replace("#", "");
    setTimeout(() => {
      const el = document.getElementById(id);
      if (el) {
        el.scrollIntoView({ behavior: "smooth" });
      }
    }, 50);
  };

  const handleNavClick = (
    e: React.MouseEvent<HTMLAnchorElement>,
    anchor: string
  ) => {
    if (!anchor.startsWith("/#")) return;
    e.preventDefault();
    const hash = anchor.substring(1);
    if (location.pathname !== "/") {
      navigate("/", { replace: false });
      setTimeout(() => {
        scrollToHash(hash);
      }, 200);
    } else {
      scrollToHash(hash);
    }
  };

  const handleOnHideMenu = () => setShowMenu(false);

  return (
    <nav className="py-5 container mx-auto px-4">
      <div className="grid grid-cols-2 md:grid-cols-3 items-center h-10">
        <div className="flex-shrink-0">
          <a href="/" className="flex items-center space-x-2">
            <img
              src="/gldt_logo.svg"
              alt="GLDT Logo"
              className="w-12 h-12"
            />
            <span className="self-center text-xl font-semibold whitespace-nowrap hidden sm:block">
              GLDT
            </span>
          </a>
        </div>
        <div className="hidden md:block justify-self-center">
          <div className="flex items-center justify-end space-x-12">
            {navItems.map(({ title, anchor }, i) => {
              const isExplorer =
                anchor === "/explorer" &&
                location.pathname.startsWith("/explorer");
              const isHash = anchor.startsWith("/#");
              return (
                <a
                  href={anchor}
                  className={`text-content font-light hover:underline uppercase ${
                    isExplorer ? "underline" : ""
                  }`}
                  key={i}
                  onClick={
                    isHash ? (e) => handleNavClick(e, anchor) : undefined
                  }
                >
                  {title}
                </a>
              );
            })}
          </div>
        </div>
        <div className="flex justify-self-end items-center">
          <a
            href="https://app.gldt.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            <button className="bg-surface/10 hover:bg-gold/10 dark:bg-surface-2 shadow rounded-full flex items-center justify-center gap-2 md:text-lg px-6 py-2 md:py-4">
              <img src="/gldt_logo.svg" alt="GLDT Logo" className="w-6 h-6" />
              <span className="whitespace-nowrap">Launch App</span>
            </button>
          </a>

          <div className="md:hidden">
            <button
              onClick={() => setShowMenu(!showMenu)}
              type="button"
              className="inline-flex items-center justify-center p-2 rounded-full hover:bg-surface-2 focus:outline-none"
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
                        <a href="/" className="flex items-center pr-4">
                          <img
                            src="/gold_dao.svg"
                            alt="Gold DAO Logo"
                            className="w-10 h-10"
                          />
                          <span className="self-center text-xl font-semibold whitespace-nowrap ml-2">
                            Gold DAO
                          </span>
                        </a>
                        <button
                          onClick={() => setShowMenu(!showMenu)}
                          type="button"
                          className="inline-flex items-center justify-center p-2 rounded-full hover:bg-surface-2 focus:outline-none"
                        >
                          <span className="sr-only">Open main menu</span>
                          <XMarkIcon className="h-6 w-6" />
                        </button>
                      </div>

                      {navItems.map(({ title, anchor }, i) => {
                        const isExplorer =
                          anchor === "/explorer" &&
                          location.pathname.startsWith("/explorer");
                        const isHash = anchor.startsWith("/#");
                        return (
                          <a
                            href={anchor}
                            onClick={
                              isHash
                                ? (e) => {
                                    handleOnHideMenu();
                                    handleNavClick(e, anchor);
                                  }
                                : handleOnHideMenu
                            }
                            className={`font-light text-content/60 hover:text-content px-3 py-2 rounded-md ${
                              isExplorer
                                ? "underline font-semibold text-gold"
                                : ""
                            }`}
                            key={i}
                          >
                            {title}
                          </a>
                        );
                      })}
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

export default NavbarTop;
