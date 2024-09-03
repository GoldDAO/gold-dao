"use client";

import { useTranslation } from "next-i18next";

const TokensInfos: React.FC = () => {
  const { t } = useTranslation("tokens");

  return (
    <div className="flex flex-col items-center justify-center gap-4 w-full mb-[96px] px-2 md:px-10 ">
      <p className="text-[30px] font-inter font-normal leading-[36px] text-center lg:w-3/4 3xl:w-1/2 my-[96px]">
        {t("description")}
      </p>
      <CardGrid />
    </div>
  );
};

interface CardProps {
  logoSrc: string;
  title: string;
  description: string;
  learnMoreLink: string;
  buyLink?: string;
  imageSrc: string;
}

const Card: React.FC<CardProps> = ({
  logoSrc,
  title,
  description,
  learnMoreLink,
  buyLink,
  imageSrc,
}) => {
  const { t } = useTranslation("tokens");

  return (
    <div className="max-w-sm overflow-hidden shadow-lg bg-white rounded-[20px]">
      <div className="px-3 p-2">
        <div className="p-4 space-y-8">
          <div className="flex flex-row items-center align-middle space-x-4 mt-6">
            <img src={logoSrc} alt={title} className="w-[40px] h-[40px]" />
            <div className="font-semibold text-[36px]">{title}</div>
          </div>
          <p className="text-gray-700 text-base h-[72px]">{description}</p>
          <div className="flex flex-col space-y-6">
            <a
              href={learnMoreLink}
              className="text-[#000000A3] underline underline-offset-[3px] hover:text-secondary duration-300 ease-in-out"
            >
              {t("learn_more")}
            </a>
            {buyLink ? (
              <a href={buyLink} className="main-button w-fit">
                {t("buy")} {title}
              </a>
            ) : (
              <a href={learnMoreLink} className="main-button-disabled w-fit">
                {t("coming_soon")}
              </a>
            )}
          </div>
        </div>
        <img
          className="w-full min-h-[215px] object-cover rounded-[20px] mt-8"
          src={imageSrc}
          alt={title}
        />
      </div>
    </div>
  );
};

const CardGrid: React.FC = () => {
  const { t } = useTranslation("tokens");

  const cards = [
    {
      logoSrc: "/static/icons/Gold-Gov.svg",
      title: t("tokens.gldgov.title"),
      description: t("tokens.gldgov.description"),
      learnMoreLink: "#",
      buyLink: "#",
      imageSrc: "/static/illustrations/Gold-Gov.svg",
    },
    {
      logoSrc: "/static/icons/gold-light-neutral-1g.svg",
      title: t("tokens.gld_nft.title"),
      description: t("tokens.gld_nft.description"),
      learnMoreLink: "#",
      buyLink: "#",
      imageSrc: "/static/illustrations/gold-light-nft.svg",
    },
    {
      logoSrc: "/static/icons/Gold-Marketcap.svg",
      title: t("tokens.gldt.title"),
      description: t("tokens.gldt.description"),
      learnMoreLink: "#",
      buyLink: "#",
      imageSrc: "/static/illustrations/Gold-GLDT.svg",
    },
    {
      logoSrc: "/static/icons/Gold-USDG.svg",
      title: t("tokens.usdg.title"),
      description: t("tokens.usdg.description"),
      learnMoreLink: "#",
      imageSrc: "/static/illustrations/Gold-USDG.svg",
    },
  ];

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-[20px]">
      {cards.map((card, index) => (
        <Card key={index} {...card} />
      ))}
    </div>
  );
};

export default TokensInfos;
