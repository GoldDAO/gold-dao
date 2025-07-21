// type LogoName =
//   | "ogy"
//   | "gldt"
//   | "gld_nft"
//   | "goldao"
//   | "icpswap"
//   | "kongswap"
//   | "lbank"
//   | "sonic"
//   | "icdex"
//   | "icp"
//   | "wtn"
//   | "ckusdt"
//   | "ckusdc";

const LOGOS: Record<string, { alt: string; src: string }> = {
  ogy: { alt: "OGY logo", src: "/ogy.svg" },
  icp: { alt: "ICP logo", src: "/icp.svg" },
  gldt: { alt: "GLDT logo", src: "/gldt.svg" },
  gld_nft: { alt: "GOLD NFT logo", src: "/gold_nft.svg" },
  goldao: { alt: "GOLD DAO logo", src: "/gold_dao.svg" },
  icpswap: { alt: "ICPSWAP logo", src: "/icpswap.svg" },
  kongswap: { alt: "KONGSWAP logo", src: "/kongswap.svg" },
  lbank: { alt: "LBANK logo", src: "/lbank.svg" },
  icdex: { alt: "ICDEX logo", src: "/icdex.svg" },
  sonic: { alt: "Sonic logo", src: "/sonic.svg" },
  wtn: { alt: "Waterneuron logo", src: "/waterneuron.svg" },
  ckusdt: { alt: "ckUSDT logo", src: "/ckusdt.svg" },
  ckusdc: { alt: "ckUSDC logo", src: "/ckusdc.svg" },
};

export const Logo = ({
  name,
  className,
}: {
  name: string;
  className?: string;
}) => {
  const pathLogos = "/logos";

  if (!(name in LOGOS)) {
    return null;
  }

  return (
    <img
      src={pathLogos + LOGOS[name].src}
      alt={LOGOS[name].alt}
      className={className ? className : "w-10 h-10"}
    />
  );
};
