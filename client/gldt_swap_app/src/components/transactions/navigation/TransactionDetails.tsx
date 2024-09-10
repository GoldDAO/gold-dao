import CopyToClipboard from "@components/shared/button/CopyToClipboard";
import { useNavigate } from "react-router-dom";

const TransactionDetails = ({ nft_id }: { nft_id: string }) => {
  const navigate = useNavigate();

  const handleGoBack = () => {
    navigate(-1);
  };
  return (
    <div className="flex items-center gap-1">
      <button className="" onClick={handleGoBack}>
        My account
      </button>
      <div>/</div>
      <>
        <div
          data-tooltip-id="tooltip"
          data-tooltip-content={nft_id}
          className="mr-2 truncate max-w-36"
        >
          {nft_id}
        </div>
        <CopyToClipboard value={nft_id} />
      </>
    </div>
  );
};

export default TransactionDetails;
