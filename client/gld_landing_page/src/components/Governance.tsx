"use client";
import { useTranslation } from 'react-i18next';

const Governance = () => {
  const { t } = useTranslation("governance");
  return (
    <section className="flex flex-col items-center justify-center gap-4 w-full mb-[96px] px-2 md:px-10 bg-[#FAF9F8]">
      <p className="font-bold text-3xl text-center">{t("title")}</p>
      <div className="space-y-4 text-center w-full md:w-1/2 py-8 text-[#000000CC]">
        <p>{t("description-1")}</p>
        <p>{t("description-2")}</p>
      </div>
      <a target="_blank" href="https://docs.gold-dao.org/v/gold-dao-whitepaper/tokenomics/sns-of-gldgov" className="main-button">{t("button")}</a>
    </section>
  );
};

export default Governance;
