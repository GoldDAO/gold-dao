import { ReactNode } from "react";

import SelectSwapMode from "@components/swap/select/SwapMode";

const From = ({ children }: { children: ReactNode }) => {
  return (
    <div className="border border-border rounded-xl p-3 md:p-6">
      <div className="flex flex-col sm:flex-row justify-between items-center">
        <div className="text-gold font-semibold">From</div>
        <SelectSwapMode />
      </div>
      <div className="mt-6">{children}</div>
    </div>
  );
};

export default From;
