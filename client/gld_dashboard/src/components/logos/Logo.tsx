export const Logo = ({
  name,
  className,
}: {
  name:
    | string
    | "ogy"
    | "gldt"
    | "gld_nft"
    | "goldao"
    | "gold_dao"
    | "gldgov"
    | "icpswap"
    | "kongswap"
    | "lbank"
    | "sonic"
    | "icdex"
    | "icp"
    | "waterneuron"
    | "wtn"
    | "ckusdt"
    | "ckusdc";
  className?: string;
}) => {
  const pathLogos = "/logos";
  const logos = {
    ogy: { alt: "OGY logo", src: "/ogy.svg" },
    icp: { alt: "ICP logo", src: "/icp.svg" },
    gldt: { alt: "GLDT logo", src: "/gldt.svg" },
    gld_nft: { alt: "GOLD NFT logo", src: "/gold_nft.svg" },
    goldao: { alt: "GOLD DAO logo", src: "/gold_dao.svg" },
    gold_dao: { alt: "GOLD DAO logo", src: "/gold_dao.svg" },
    gldgov: { alt: "GLDGov logo", src: "/gldgov.svg" },
    icpswap: { alt: "ICPSWAP logo", src: "/icpswap.svg" },
    kongswap: { alt: "KONGSWAP logo", src: "/kongswap.svg" },
    lbank: { alt: "LBANK logo", src: "/lbank.svg" },
    icdex: { alt: "ICDEX logo", src: "/icdex.svg" },
    sonic: { alt: "Sonic logo", src: "/sonic.svg" },
    waterneuron: { alt: "Waterneuron logo", src: "/waterneuron.svg" },
    wtn: { alt: "Waterneuron logo", src: "/waterneuron.svg" },
    ckusdt: { alt: "ckUSDT logo", src: "/ckusdt.svg" },
    ckusdc: { alt: "ckUSDC logo", src: "/ckusdc.svg" },
  };
  return (
    <img
      src={pathLogos + logos[name].src}
      alt={logos[name].alt}
      className={className ? className : "w-10 h-10"}
    />
  );
};
