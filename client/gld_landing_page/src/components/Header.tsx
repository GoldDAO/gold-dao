"use client";

import Image from "next/image";
import { useState } from "react";
import { useTranslation } from "next-i18next";
import MenuIcon from "../../public/static/icons/MenuIcon";
import XIcon from "../../public/static/icons/XIcon";

const Header = () => {
  const [isOpen, setIsOpen] = useState(false);
  const { t } = useTranslation("header");

  const toggleMenu = () => {
    setIsOpen(!isOpen);
  };

  const links = [
    { name: t("links.gldt"), href: "#" },
    { name: t("links.gld_swap"), href: "#" },
    { name: t("links.gld_nft"), href: "#" },
  ];

  return (
    <header className="flex relative z-50 justify-between w-full items-center border bg-white/10 border-[#FAF9F826] rounded-full m-4 p-1 text-sm max-w-screen-xl bg-clip-padding backdrop-filter backdrop-blur-2xl text-main">
      <div className="flex flex-row flex-wrap items-center gap-2 pl-1">
        <Image
          src="/static/icons/GoldDAO.svg"
          alt="Bity"
          width={28}
          height={28}
        />
        <h1 className="text-2xl font-bold ">{t("title")}</h1>
      </div>

      <button className="md:hidden z-20">
        {isOpen ? (
          <XIcon className="w-8 h-8 text-main" onClick={toggleMenu} />
        ) : (
          <MenuIcon className="w-8 h-8 text-main" onClick={toggleMenu} />
        )}
      </button>

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

      <button className="main-button font-semibold hidden md:block">
        {t("buttons.dashboard")}
      </button>

      {isOpen && (
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
              {t("buttons.dashboard")}
            </button>
          </nav>
        </div>
      )}
    </header>
  );
};

export default Header;
