"use client";

import Image from "next/image";
import { useTranslation } from 'react-i18next';

interface Card {
  title: string;
  tag: string;
  description: string;
  imageSrc?: string;
  videoSrc?: string;
  points: string[];
}

const TokensCards = () => {
  const { t } = useTranslation("cards");

  const cards: Card[] = t("cards", { returnObjects: true }) as Card[];

  return (
    <div className="flex flex-col items-center justify-center gap-[24rem] md:gap-[14rem] px-2 w-[calc(100%-45px)] pt-[96px] 3xl:max-w-[90rem] mb-[96px] bg-[#FBF8F1]">
      {cards &&
        cards.map((card, index) => (
          <div
            key={index}
            className="card shadow-lg bg-white rounded-[20px] flex flex-col md:flex-row items-center max-h-[780px] md:max-h-[612px] w-full">
            {/* Text Section*/}
            <div className="flex flex-col md:w-1/2 p-8 md:py-16 md:px-16 2xl:px-32">
              <div className="bg-[#F7EED7] text-[#B89143] rounded-full w-fit px-4 py-1 mb-4">
                {card.tag}
              </div>
              <div className="font-semibold text-3xl mb-4 pt-8">
                {card.title}
              </div>
              <p className="text-base mb-4">{card.description}</p>
              {card.points.map((point, pointIndex) => (
                <div key={pointIndex} className="my-1">
                  <span className="text-[16px]">
                    {pointIndex + 1}. {point}
                  </span>
                </div>
              ))}
            </div>
            {/* Image / Video Section */}
            <div className="relative w-full md:w-1/2 h-[780px] md:h-[612px] ">
              {card.videoSrc && (
                <div className="">
                  <video
                    src={card.videoSrc}
                    autoPlay
                    loop
                    muted
                    playsInline
                    className="rounded-b-[20px] md:rounded-r-[20px] md:rounded-b-none object-cover w-full h-full absolute"
                    controlsList="nodownload"
                  />
                </div>
              )}

              {card.imageSrc && (
                <div className="">
                  <Image
                    src={card.imageSrc}
                    alt={card.title}
                    fill
                    className="rounded-b-[20px] md:rounded-r-[20px] md:rounded-b-none object-cover"
                    sizes="(max-width: 768px) 100vw, 50vw"
                  />
                </div>
              )}
            </div>
          </div>
        ))}
    </div>
  );
};

export default TokensCards;
