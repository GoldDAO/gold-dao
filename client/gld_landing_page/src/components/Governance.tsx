"use client";
import { useTranslation } from "react-i18next";

const Governance = () => {
  const { t } = useTranslation("governance");
  return (
    <div className="flex flex-col items-center justify-center gap-4 w-full mb-[96px] px-2 md:px-10 bg-[#FAF9F8]">
      <p className="font-bold text-3xl text-center">{t("title")}</p>
      <div className="space-y-4 text-center w-full md:w-1/2 py-8">
        <p>{t("description-1")}</p>
        <p>{t("description-2")}</p>
      </div>
      <button className="main-button">{t("button")}</button>
    </div>
  );
};

export default Governance;
