import { ReactNode } from "react";
import { Link } from "react-router-dom";
import { Button, Skeleton } from "@components/ui";
// import { useLedgerMetadata } from "@hooks/ledger";
import { useGLDNFTLocked } from "@hooks/gld_nft/useGLDNFTLocked";

const Badge = ({
  className,
  title,
  value,
}: {
  className?: string;
  title: ReactNode;
  value: ReactNode;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center gap-2 bg-surface-1 border border-surface-2 rounded-full pl-4 pr-1 py-2">
        <div className="text-sm">{title}</div>
        <div className="rounded-full px-2 py-1 bg-burgundy text-white font-semibold">
          {value}
        </div>
      </div>
    </div>
  );
};

const Hero = () => {
  // const { data: GLDTMetadata, isSuccess: isSuccessGLDTMetadata } =
  //   useLedgerMetadata({ ledger: "GLDT" });

  const { data: NFTLocked, isSuccess: isSuccessNFTLocked } = useGLDNFTLocked();

  return (
    <div className="container mx-auto flex flex-col flex-grow px-4">
      <div className="flex flex-1 flex-col gap-4 md:gap-8 items-center justify-center">
        <div className="text-center text-4xl md:text-7xl">
          <h1 className="mb-2">Instantly own</h1>
          <h1 className="text-gold font-semibold">tokenized Swiss Gold</h1>
        </div>

        <div className="max-w-96 text-center text-content/80 gap-1 text-xl md:text-2xl">
          GLDT turns{" "}
          <span className="text-content font-semibold">physical Gold</span> into
          a <span className="text-content font-semibold">digital asset</span>{" "}
          secured in Swiss vaults, available anytime.
        </div>

        <Link
          to="https://app.gldt.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          <Button className="rounded-full flex items-center justify-center gap-2 px-4 md:px-6 py-3 md:py-4 md:text-lg">
            <img src="/gldt_logo.svg" alt="GLDT Logo" className="w-6 h-6" />
            Buy GLDT now
          </Button>
        </Link>
      </div>
      <div className="mt-auto flex flex-col md:flex-row items-center justify-center gap-4 pb-8 md:pb-16">
        <Badge
          title={
            <div>
              Total Gold <span className="font-semibold">locked</span>
            </div>
          }
          value={
            isSuccessNFTLocked ? (
              `${Number(NFTLocked).toFixed(2)} kg`
            ) : (
              <Skeleton className="w-32" />
            )
          }
        />
      </div>
    </div>
  );
};

export default Hero;
