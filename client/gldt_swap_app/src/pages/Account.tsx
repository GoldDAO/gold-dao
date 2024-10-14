import { useAuth } from "@auth/index";
import TotalCountNfts from "@components/account/TotalCountNfts";
import TotalSwappedGLDT from "@components/account/TotalSwappedGLDT";
import TotalWeightNfts from "@components/account/TotalWeightNfts";

import TokenBalance from "@components/account/token-balance/TokenBalance";
import YourNfts from "@components/account/your-nfts/YourNfts";

import OngoingTransactions from "@components/transactions/list/ongoing/Ongoing";
import PastTransactions from "@components/transactions/list/past/Past";

// todo better handle isConnected state

export const Account = () => {
  const { isConnected } = useAuth();

  return (
    <div className="container mx-auto mt-4 sm:mt-8">
      <div className="mb-8">
        <div className="text-4xl font-semibold text-gold">My Account</div>
        <div className="text-4xl">Overview</div>
      </div>

      <div className="flex flex-col sm:flex-row sm:items-center gap-4 justify-between mb-4">
        <TotalCountNfts className="w-full" />
        <TotalWeightNfts className="w-full" />
        <TotalSwappedGLDT className="w-full blur-sm" />
      </div>

      <TokenBalance className="mb-4" />

      <YourNfts className="mb-4" />

      <div className="bg-surface rounded-xl border border-border px-6 py-4">
        <div>My transactions</div>
        <div className="mt-6">
          <OngoingTransactions />
        </div>
        <div className="mt-6">{isConnected && <PastTransactions />}</div>
      </div>
    </div>
  );
};
