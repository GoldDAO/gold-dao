import { useNavigate } from "react-router-dom";

const BtnClaimToken = () => {
  const navigate = useNavigate();

  const handleOnClick = () => {
    navigate("");
  };

  return (
    <div
      className="border border-border text-gold font-semibold px-4 py-1 cursor-pointer rounded-lg"
      onClick={handleOnClick}
    >
      Claim token
    </div>
  );
};

export default BtnClaimToken;
