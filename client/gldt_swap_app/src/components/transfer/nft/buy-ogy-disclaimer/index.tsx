import { Link } from "react-router-dom";
import { Button } from "@components/ui";

import { useTransferProceedNft } from "@context/transfer/proceed-nft";

const NotEnoughOGYDisclaimer = ({ className }: { className?: string }) => {
  const { state } = useTransferProceedNft();
  const { totalTransferFee, balance, countSelectedNfts } = state;

  return (
    <div className={className}>
      <div className="border border-orange-500 bg-orange-500/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl text-center">
        <div className="mb-6 text-orange-500">
          <div className="font-semibold mb-2">
            You don't have enough OGY to process
          </div>
          <div>
            To transfer{" "}
            <span className="font-semibold">{countSelectedNfts} GLD NFT</span>,
            you need to buy at least{" "}
            <span className="font-semibold">{totalTransferFee.string} OGY</span>{" "}
            to be able to pay the fee. Your current balance is{" "}
            <span className="font-semibold">{balance} OGY</span>
          </div>
        </div>
        <div>
          <Link
            to="https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=lkwrt-vyaaa-aaaaq-aadhq-cai"
            target="_blank"
            rel="noopener noreferrer"
          >
            <Button>Buy OGY</Button>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default NotEnoughOGYDisclaimer;
