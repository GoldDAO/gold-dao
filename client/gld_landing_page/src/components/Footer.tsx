"use client";
import { useTranslation } from "react-i18next";
const Footer = () => {
  const { t } = useTranslation("footer");
  return (
    <div className="flex flex-col w-full bg-black py-2 px-16 h-[50px] justify-center">
      <p className="text-xs font-normal text-white ">{t("from")}</p>
    </div>
  );
};

export default Footer;
