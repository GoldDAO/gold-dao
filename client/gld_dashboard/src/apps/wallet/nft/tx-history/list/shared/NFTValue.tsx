import { BlockTx } from "@services/nft/utils/interfaces";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";

const NFTValue = ({ tx }: { tx: BlockTx }) => {
  const goldPrice = useFetchPriceGold({});

  const calculatedValue =
    tx.value && goldPrice.data ? tx.value * goldPrice.data : 0;

  return (
    <div>
      <div className="text-right text-lg">{tx.weight}</div>
      <div className="text-content/60 text-sm text-right">
        {goldPrice.isSuccess ? (
          <>
            $
            <NumberToLocaleString value={calculatedValue} decimals={2} />
          </>
        ) : (
          <div>Loading...</div>
        )}
      </div>
    </div>
  );
};

export default NFTValue;
