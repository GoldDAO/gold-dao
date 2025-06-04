const RenderValueNFT = ({ value }: { value: number }) => {
  const getCollectionNameByValue = (value: number) => {
    switch (value) {
      case 1:
        return "1G";
      case 10:
        return "10G";
      case 100:
        return "100G";
      case 1000:
        return "1KG";
      default:
        return "1G";
    }
  };

  return (
    <div className="flex items-center gap-1">
      <div className="w-[32px] flex justify-center items-center">
        <img
          className="flex-none h-8"
          src={`/gold-bars/${getCollectionNameByValue(value)}.svg`}
        />
      </div>
      <div className="text-left">
        <div className="font-semibold">{getCollectionNameByValue(value)}</div>
        <div className="text-content/60 text-xs">GOLD</div>
      </div>
    </div>
  );
};

export default RenderValueNFT;
