"use client";

import { useTranslation } from "react-i18next";

const WhyInfos: React.FC = () => {
  const { t } = useTranslation("why");

  return (
    <div className="bg-white">
      {/* Section Title and Description */}
      <div className="flex flex-col md:flex-row w-full border-y border-secondary">
        <div className="md:w-1/2 py-16 md:py-0 md:mt-16 border-b md:border-0 border-secondary">
          <p className="text-[30px] text-center">
            <span className="font-bold">{t("titleBold")} </span>
            {t("titleRegular")}
          </p>
        </div>
        <div className="hidden md:block border-r w-0.5 border-secondary"></div>
        <div className="md:w-1/2 my-16 ">
          <p className="text-[16px] text-left justify-left w-3/4 md:w-8/12 3xl:w-5/12 mx-auto gap-5">
            {t("description-1")}
          </p>
          <p className="mt-4 text-[16px] text-left justify-left  w-3/4 md:w-8/12 3xl:w-5/12 mx-auto gap-5">
            {t("description-2")}
          </p>
        </div>
      </div>

      {/* Section Features */}
      <div className="w-full flex flex-col md:flex-row border-b  border-secondary">
        {["decentralized", "ownership", "crossChain", "audited"].map(
          (feature, index) => {
            const isLast =
              index ===
              ["decentralized", "ownership", "crossChain", "audited"].length -
                1;
            return (
              <div
                key={feature}
                className={`${!isLast && "md:border-r border-secondary"} border-b md:border-b-0 border-secondaryfont-sans text-2xl font-semibold   leading-[32px] text-secondary bg-black md:w-1/4 text-center content-center py-2`}>
                {t(`features.${feature}`)}
              </div>
            );
          }
        )}
      </div>
    </div>
  );
};

export default WhyInfos;
