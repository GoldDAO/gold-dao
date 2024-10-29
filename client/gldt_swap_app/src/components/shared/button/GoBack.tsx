import { useNavigate } from "react-router-dom";
import { ArrowUturnLeftIcon } from "@heroicons/react/24/outline";

const GoBack = ({ className = "" }: { className?: string }) => {
  const navigate = useNavigate();

  const handleGoBack = () => {
    navigate(-1);
  };
  return (
    <div className={className}>
      <button onClick={handleGoBack} className="flex items-center gap-3">
        <ArrowUturnLeftIcon className="h-8 w-8 text-gold" />
        <div className="font-semibold">Go back</div>
      </button>
    </div>
  );
};

export default GoBack;
