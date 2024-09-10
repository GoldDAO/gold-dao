import { useNavigate } from "react-router-dom";

const BtnNavigateToTxDetails = ({
  nft_id,
  index,
}: {
  nft_id: string;
  index: string;
}) => {
  const navigate = useNavigate();

  const handleOnClick = () => {
    navigate(`/swap/account/transactions/${nft_id}?index=${index}`);
  };

  return (
    <div
      className="border border-border text-gold font-semibold px-4 py-1 cursor-pointer rounded-lg"
      onClick={handleOnClick}
    >
      Transaction details
    </div>
  );
};

export default BtnNavigateToTxDetails;
