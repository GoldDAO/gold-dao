import { useNavigate } from "react-router-dom";
import { Button } from "@components/ui";
import { useReverseSwapProceed } from "@context/index";

const Success = () => {
  const navigate = useNavigate();
  const { handleClose } = useReverseSwapProceed();

  const handleOnClickGoToTxView = () => {
    handleClose();
    navigate("/swap/account#active-swaps");
  };

  return (
    <div className="text-center">
      <div className="flex flex-col justify-center items-center rounded-xl">
        <video autoPlay={true} loop={true} className="scale-x-[-1]">
          <source
            src="https://daolink-gold-dao-website-medias.sos-ch-gva-2.exo.io/GLDNFT2GLDT.webm"
            type="video/webm"
          />
          Your browser does not support the video tag.
        </video>
        <div className="mb-6 font-semibold text-gold">
          <div className="text-xl font-semibold mb-4">
            Great, you've successfully initiated a swap!
          </div>
          <div className="text-sm">
            You can see the advancement in your transactions history.
          </div>
        </div>
      </div>

      <Button className="mt-8 w-full" onClick={handleOnClickGoToTxView}>
        Go to transactions history
      </Button>
    </div>
  );
};

export default Success;
