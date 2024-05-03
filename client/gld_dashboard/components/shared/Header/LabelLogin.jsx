"use client";

import { shortPrincipal } from "../../../utils/parsers";
import { useSession } from "../../../hooks/useSession";

const LabelLogin = () => {
  const { principal, isConnected } = useSession();

  if (!isConnected) return;

  return (
    <div className="border-[#D3B871] border-[0.5px] rounded-full bg-[#F3F3F3] px-6 py-3 shadow-[0_0_12px_0_#00000026]">
      <p className="text-[18px]">
        My Principal ID
        <span className="font-bold">
          {shortPrincipal(
            principal || "3px7p-abe4z-r4yl3-gek3i-hso7z-hj27l-vmjho-ytvbj-kyaze-u4btl-fqe"
          )}
        </span>
      </p>
    </div>
  );
};

export default LabelLogin;
