"use client";

import Image from "next/image";
import { useTranslation } from 'react-i18next';
const Partners = () => {
  const { t } = useTranslation("partners");
  const partners = [
    {
      name: "Metalor",
      logo: "/static/icons/metalor.svg",
      url: "https://golddao.org/",
    },
    {
      name: "Origyn",
      logo: "/static/icons/ORIGYN.svg",
      url: "https://golddao.org/",
    },
    {
      name: "KPMG",
      logo: "/static/icons/KPMG.svg",
      url: "https://golddao.org/",
    },
    {
      name: "Loomis",
      logo: "/static/icons/Loomis.svg",
      url: "https://golddao.org/",
    },
    {
      name: "ICP",
      logo: "/static/icons/ICP-bw.svg",
      url: "https://golddao.org/",
    },
    {
      name: "Bity",
      logo: "/static/icons/Bity.svg",
      url: "https://golddao.org/",
    },
  ];
  return (
    <div className="flex flex-col items-center justify-center gap-8 w-full py-16  mt-16 mb-[96px] px-2 md:px-10 bg-white border border-y-secondary ">
      <p className="text-[rgba(0,0,0,0.64)] text-center font-inter text-[16px] font-bold leading-[32px] tracking-[2.56px] uppercase">
        {t("title")}
      </p>
      <div className="flex flex-row flex-wrap justify-around gap-4 w-full">
        {partners.map((partner, index) => (
          <a
            href={partner.url}
            target="_blank"
            rel="noreferrer"
            key={partner.name}
            className="group">
            <Image
              src={partner.logo}
              alt={partner.name}
              width={156}
              height={100}
              className="group-hover:opacity-80 transition-all duration-500 ease-in-out"
            />
          </a>
        ))}
      </div>
    </div>
  );
};

export default Partners;
