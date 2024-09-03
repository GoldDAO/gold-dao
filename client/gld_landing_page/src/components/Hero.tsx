"use client"

import { useTranslation } from "next-i18next"
const Hero = () => {
  const { t } = useTranslation("hero")
  return (
    <>
      <video
        autoPlay={true}
        loop={true}
        muted={true}
        playsInline={true}
        className="absolute inset-0 w-full h-full object-cover"
        src="/videos/Gold_DAO_bg_video.mp4"
      />
      <div className="relative text-center top-6 md:top-36 w-full 3xl:top-44 xl:w-3/4 3xl:w-1/2 4xl:top-36">
        <h1
          className="text-[60px] md:text-[82px] font-inter font-bold text-white leading-[90px] text-shadow-lg"
          style={{
            textShadow:
              "0px 10px 15px rgba(0, 0, 0, 0.10), 0px 4px 6px rgba(0, 0, 0, 0.05)",
          }}>
          {t("title")}
        </h1>
        <p
          className="text-[40px] md:text-[82px] font-inter font-light leading-[90px] text-[rgba(0,0,0,0.80)]"
          style={{
            textShadow:
              "0px 10px 15px rgba(0, 0, 0, 0.10), 0px 4px 6px rgba(0, 0, 0, 0.05)",
          }}>
          {t("subtitle")}
        </p>
        <div className="mt-10 sm:mt-[64px] flex-col space-y-6  xl:space-y-0  xl:w-full flex lg:flex-row justify-around items-center">
          <div className="flex h-10 px-4 pl-2 justify-center items-center rounded-3xl border gap-[8px] border-[#D3B872] bg-white">
            <span className="w-6 h-6 flex-shrink-0 bg-gradient-to-b  from-[#FFFCF2] to-[#DDD5BC] rounded-full drop-shadow-[0_10px_60px_rgba(64,56,48,0.05)] flex items-center justify-center">
              <img
                src="/static/icons/Gold-Light-1g.svg"
                className="w-[13px] h-[19px] flex-shrink-0"
                alt="Icon"
              />
            </span>
            <span className="font-inter font-normal leading-[16px] text-[#262C2E]">
              {t("total_gold_locked")}
            </span>
            <span className="h-full w-0.5 bg-[#D3B872] rounded-3xl mx-[8px]"></span>
            <span className="font-bold">{t("total_gold_value")}</span>
          </div>
          <div className="flex h-10 px-4 pl-2 justify-center items-center rounded-3xl border gap-[8px] border-[#D3B872] bg-white">
            <img
              src="/static/icons/Gold-Marketcap.svg"
              className="w-[24px] h-[24px] flex-shrink-0"
              alt="Icon"
            />
            <span className="font-inter font-normal leading-[16px] text-[#262C2E]">
              {t("gldt_marketcap")}
            </span>
            <span className="h-full w-0.5 bg-[#D3B872] rounded-3xl mx-[8px]"></span>
            <span className="font-bold">{t("gldt_marketcap_value")}</span>
          </div>
        </div>
      </div>
    </>
  );
};

export default Hero;
