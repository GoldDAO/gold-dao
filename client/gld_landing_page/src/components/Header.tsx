/* eslint-disable react-hooks/rules-of-hooks */
"use client";

import Image from "next/image";
import { useEffect, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import MenuIcon from "../../public/static/icons/MenuIcon";
import XIcon from "../../public/static/icons/XIcon";

interface Link {
  name: string;
  href: string;
}

const Header: React.FC = () => {
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const { t } = useTranslation("header");

  const toggleMenu = (): void => {
    setIsOpen(!isOpen);
  };

  const links: Link[] = [
    { name: t("links.gldt"), href: "#" },
    { name: t("links.gld_swap"), href: "#" },
    { name: t("links.gld_nft"), href: "#" },
  ];

  const menuRef = useRef<HTMLDivElement>(null); // Référence pour le menu

  // Effet pour détecter les clics à l'extérieur du menu
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsOpen(false); // Ferme le menu si on clique en dehors
      }
    };

    // Ajoute un écouteur d'événements lors du montage
    if (isOpen) {
      document.addEventListener("mousedown", handleClickOutside);
    }

    // Nettoie l'écouteur lors du démontage
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [isOpen]);

  return (
    <header className="flex z-50 fixed justify-between items-center  my-4 p-1 text-sm 4xl:max-w-screen-xl w-[calc(100%-14px)] md:w-[calc(100%-60px)] mx-auto bg-clip-padding backdrop-filter text-main rounded-[100px] border border-[#FAF9F880] bg-[rgba(255,255,255,0.10)] shadow-[0px_5px_30px_0px_rgba(135,88,29,0.10)] backdrop-blur-[30px]">
      <LogoSection />
      <MenuButton isOpen={isOpen} toggleMenu={toggleMenu} />
      <Navigation links={links} />
      <div className="hidden md:block">
        <DashboardButton />
      </div>
      <MobileMenu
        isOpen={isOpen}
        links={links}
        toggleMenu={toggleMenu}
        menuRef={menuRef}
      />
    </header>
  );
};

const LogoSection: React.FC = () => {
  const { t } = useTranslation("header");
  return (
    <div className="flex flex-row flex-wrap items-center gap-2 pl-4">
      <Image
        src="/static/icons/GoldDAO.svg"
        alt="Gold DAO"
        width={28}
        height={28}
      />
      <h1 className="text-xl font-bold">{t("title")}</h1>
    </div>
  );
};

interface MenuButtonProps {
  isOpen: boolean;
  toggleMenu: () => void;
}

const MenuButton: React.FC<MenuButtonProps> = ({ isOpen, toggleMenu }) => (
  <button className="md:hidden z-20">
    {isOpen ? (
      <XIcon className="w-8 h-8 text-main" onClick={toggleMenu} />
    ) : (
      <MenuIcon className="w-8 h-8 text-main" onClick={toggleMenu} />
    )}
  </button>
);

interface NavigationProps {
  links: Link[];
}

const Navigation: React.FC<NavigationProps> = ({ links }) => (
  <nav className="items-center gap-[64px] hidden md:flex">
    {links.map((link) => (
      <a
        key={link.name}
        href={link.href}
        target="_blank"
        rel="noopener noreferrer"
        className="font-medium text-main hover:underline hover:underline-offset-[14px]">
        {link.name}
      </a>
    ))}
  </nav>
);

const DashboardButton: React.FC = () => (
  <button className="main-button text-white font-semibold text-[16px] leading-[24px] md:block hidden">
    {useTranslation("header").t("buttons.dashboard")}
  </button>
);

interface MobileMenuProps {
  isOpen: boolean;
  links: Link[];
  toggleMenu: () => void;
  menuRef: React.RefObject<HTMLDivElement>;
}

const MobileMenu: React.FC<MobileMenuProps> = ({
  isOpen,
  links,
  toggleMenu,
  menuRef,
}) => {
  if (!isOpen) return null;

  return (
    <div
      ref={menuRef}
      className="absolute top-[70px] right-0 mt-2 w-full bg-black shadow-lg md:hidden rounded-lg">
      <nav className="flex flex-col items-center p-4">
        {links.map((link) => (
          <a
            key={link.name}
            href={link.href}
            onClick={toggleMenu}
            className="w-full text-center py-2 text-white font-medium border-b border-gray-200">
            {link.name}
          </a>
        ))}
        <button className="secondary-button font-semibold w-full mt-4">
          {useTranslation("header").t("buttons.dashboard")}
        </button>
      </nav>
    </div>
  );
};

export default Header;
