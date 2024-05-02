"use client";

import LoginButton from "../shared/Header/LoginButton";
import RewardsBalance from "../shared/rewards/RewardsBalance";
import RewardsNeurons from "../shared/rewards/RewardsNeurons";
import { useSession } from "../../hooks/useSession";
import { useState } from "react";

const RewardsContent = () => {
  const { isConnected, isConnecting } = useSession();
  const [ogy, setOgy] = useState({ loading: true, amount: 0 });
  const [icp, setIcp] = useState({ loading: true, amount: 0 });
  const [gold, setGold] = useState({ loading: true, amount: 0 });

  return (
    <div className="px-2 sm:px-3 mb-12 sm:mb-48">
      {isConnecting ? (
        <section className="flex flex-col items-center justify-center gap-8 sm:min-h-[600px] min-h-[400px]">
          <h3 className="font-bold sm:text-2xl text-lg text-center">Connecting...</h3>
        </section>
      ) : isConnected ? (
        <div className="flex justify-start flex-col gap-3 ">
          <RewardsBalance
            setOgy={setOgy}
            setIcp={setIcp}
            setGold={setGold}
            ogy={ogy}
            icp={icp}
            gold={gold}
          />
          <RewardsNeurons
            setOgy={setOgy}
            setIcp={setIcp}
            setGold={setGold}
            ogy={ogy}
            icp={icp}
            gold={gold}
          />
        </div>
      ) : (
        <section className="flex flex-col items-center justify-center gap-8 sm:min-h-[600px] min-h-[100vh]">
          <h3 className="font-bold sm:text-2xl text-lg text-center px-5">
            Please connect your wallet to access to your rewards.
          </h3>
          <section>
            <LoginButton />
          </section>
        </section>
      )}
    </div>
  );
};

export default RewardsContent;
