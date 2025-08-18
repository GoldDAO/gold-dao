import { Link } from "react-router-dom";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const DisclaimerInsufficientOGYFunds = ({
  totalNFTSelected,
  txFee,
  balanceOGY,
  className,
}: {
  totalNFTSelected: number;
  txFee: number;
  balanceOGY: number;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="border border-warning bg-warning/5 py-8 px-4 flex flex-col justify-center items-center rounded-xl text-center">
        <div className="mb-6 text-warning">
          <div className="mb-2">You don't have enough OGY to process</div>
          <div>
            To transfer{" "}
            <span className="font-semibold">{totalNFTSelected} GLD NFT</span>,
            you need to buy at least{" "}
            <span className="font-semibold">
              <NumberToLocaleString
                value={totalNFTSelected * txFee}
                decimals={3}
              />{" "}
              OGY
            </span>{" "}
            to be able to pay the fee. Your current balance is{" "}
            <span className="font-semibold">
              <NumberToLocaleString value={balanceOGY} decimals={3} /> OGY
            </span>
          </div>
        </div>
        <div>
          <Link
            to="https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=lkwrt-vyaaa-aaaaq-aadhq-cai"
            target="_blank"
            rel="noopener noreferrer"
          >
            <BtnPrimary>Buy OGY</BtnPrimary>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default DisclaimerInsufficientOGYFunds;
