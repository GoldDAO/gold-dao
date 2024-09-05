/* eslint-disable react-hooks/rules-of-hooks */
"use client";

import Image from "next/image";
import { useState } from "react";
import { useTranslation } from "next-i18next";
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

  return (
    <header className="flex z-50 fixed justify-between items-center border bg-white/20 border-[#FAF9F826] rounded-full my-4 p-1 text-sm 4xl:max-w-screen-xl w-[calc(100%-60px)] mx-auto bg-clip-padding backdrop-filter backdrop-blur-lg text-main">
      <LogoSection />
      <MenuButton isOpen={isOpen} toggleMenu={toggleMenu} />
      <Navigation links={links} />
      <DashboardButton />
      <MobileMenu isOpen={isOpen} links={links} toggleMenu={toggleMenu} />
    </header>
  );
};

const LogoSection: React.FC = () => (
  <div className="flex flex-row flex-wrap items-center gap-2 pl-4">
    <Image
      src="/static/icons/GoldDAO.svg"
      alt="Gold DAO"
      width={28}
      height={28}
    />
    <h1 className="text-xl font-bold">{useTranslation("header").t("title")}</h1>
  </div>
);

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
  <button className="main-button font-semibold hidden md:block">
    {useTranslation("header").t("buttons.dashboard")}
  </button>
);

interface MobileMenuProps {
  isOpen: boolean;
  links: Link[];
  toggleMenu: () => void;
}

const MobileMenu: React.FC<MobileMenuProps> = ({
  isOpen,
  links,
  toggleMenu,
}) => {
  if (!isOpen) return null;

  return (
    <div className="absolute top-[70px] right-0 mt-2 w-full bg-black shadow-lg md:hidden rounded-lg">
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
