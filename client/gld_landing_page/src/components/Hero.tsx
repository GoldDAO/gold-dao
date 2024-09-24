/* eslint-disable @next/next/no-img-element */
"use client";

import { useTranslation } from "react-i18next";
import Image from "next/image";
import { useQuery } from "@tanstack/react-query";
import { fetchTokenMetrics, TokenMetrics } from "../lib/fetchTokenMetrics";
import { useEffect, useState } from "react";

interface InfoCardProps {
  iconSrc: string;
  iconAlt: string;
  text: string;
  value: string;
  loading: boolean;
}

const InfoCard = ({
  iconSrc,
  iconAlt,
  text,
  value,
  loading,
}: InfoCardProps) => (
  <section className="flex h-10 px-4 pl-2 min-w-full md:min-w-0 justify-center items-center rounded-3xl border gap-[8px] mx-2 border-[#D3B872] bg-white">
    {iconSrc && (
      <Image
        src={iconSrc}
        alt={iconAlt}
        width={24}
        height={24}
        className="w-[24px] h-[24px] flex-shrink-0"
      />
    )}
    <span className="font-inter font-normal leading-[16px] text-[#262C2E]">
      {text}
    </span>
    <span className="h-full w-0.5 bg-[#D3B872] rounded-3xl mx-[8px]"></span>
    {loading ? (
      <span className="loading-skeleton"></span>
    ) : (
      <span className="font-bold">{value}</span>
    )}
  </section>
);

const Hero = () => {
  const [videoLoaded, setVideoLoaded] = useState(false);
  const { data, isLoading, error } = useQuery<TokenMetrics>({
    queryKey: ["tokenMetrics"],
    queryFn: fetchTokenMetrics,
  });

  const { t } = useTranslation("hero");

  const totalGoldLockedKg = data
    ? (parseFloat(data!.total_gold_grams) / 1000).toFixed(2)
    : null; // Convert grams to kg
  const marketCapUSD = data
    ? Math.ceil(parseFloat(data!.tvl)).toLocaleString("en-US")
    : null;

  useEffect(() => {
    const videoElement = document.getElementById(
      "hero-video"
    ) as HTMLVideoElement;
    if (videoElement) {
      videoElement.addEventListener("canplaythrough", () => {
        setVideoLoaded(true);
      });
    }

    return () => {
      if (videoElement) {
        videoElement.removeEventListener("canplaythrough", () => {
          setVideoLoaded(true);
        });
      }
    };
  }, []);

  return (
    <div className="h-[85vh] md:h-[75vh] w-full flex flex-col items-center justify-center px-2 md:px-10 ">
      {!videoLoaded && (
        <img
          src="/static/backgrounds/hero_bg_video.webp"
          alt="Poster"
          className="absolute inset-0 w-full h-[85vh] md:h-3/4 object-cover"
        />
      )}
      <video
        id="hero-video"
        autoPlay
        loop
        muted
        preload="auto"
        playsInline
        className={`absolute inset-0 w-full h-[85vh] md:h-3/4 object-cover`}
        src="/videos/Gold_DAO_bg.mp4"
        poster="/backgrounds/bg_video.svg"
      />
      <div className="relative text-center">
        <h1
          className="text-[53px] md:text-[82px] font-inter font-bold text-white leading-[90px] text-shadow-lg md:max-w-2xl mx-auto"
          style={{
            textShadow:
              "0px 10px 15px rgba(0, 0, 0, 0.10), 0px 4px 6px rgba(0, 0, 0, 0.05)",
          }}>
          {t("title")}
        </h1>
        <p
          className="text-[40px] md:text-[82px] font-inter font-light leading-[90px] text-[rgba(0,0,0,0.80)]  w-3/4 md:w-full mx-auto  md:max-w-2xl"
          style={{
            textShadow:
              "0px 10px 15px rgba(0, 0, 0, 0.10), 0px 4px 6px rgba(0, 0, 0, 0.05)",
          }}>
          {t("subtitle")}
        </p>
        <div className="mt-10 sm:mt-[64px] flex-col space-y-6 xl:space-y-0 xl:w-full flex lg:flex-row justify-around items-center">
          <InfoCard
            iconSrc="/static/icons/Gold-Light-1g.svg"
            iconAlt="Total Gold Icon"
            text={t("total_gold_locked")}
            value={`${totalGoldLockedKg} kg`}
            loading={isLoading}
          />
          <InfoCard
            iconSrc="/static/icons/Gold-Marketcap.svg"
            iconAlt="Marketcap Icon"
            text={t("gldt_marketcap")}
            value={`$${marketCapUSD}`}
            loading={isLoading}
          />
        </div>
      </div>
    </div>
  );
};

export default Hero;
