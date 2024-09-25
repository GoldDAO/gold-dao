const NftWeight = ({
  nftWeight,
  className,
}: {
  nftWeight: number;
  className?: string;
}) => {
  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-4">
        <div className="mb-2 font-light text-content/60 text-center sm:text-left">
          Total of NFTs weight
        </div>
        <div className="flex items-center justify-center sm:justify-start gap-4">
          <img className="flex-none h-8" src={`/gold-bars/${nftWeight}g.svg`} />
          <div className="font-semibold text-4xl">{nftWeight}g</div>
        </div>
      </div>
    </div>
  );
};

export default NftWeight;
