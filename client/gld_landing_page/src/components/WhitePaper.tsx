"use client";

import { useTranslation } from 'react-i18next';

const WhitePaper = () => {
  const { t } = useTranslation("whitepaper");

  return (
    <div
      style={{
        backgroundImage: "url('/static/backgrounds/bar_texture_full_3.svg')",
      }}
      className="flex flex-col items-center justify-center w-full px-2 bg-cover bg-center md:px-10 bg-white py-24 border-y border-secondary">
      <p className="text-6xl font-bold text-secondary ">{t("title")}</p>
      <p className="text-[#161819] text-[60px] text-center font-normal leading-[60px] ">
        {t("description")}
      </p>
      <button className="main-button mt-6">{t("button")}</button>
    </div>
  );
};

export default WhitePaper;
