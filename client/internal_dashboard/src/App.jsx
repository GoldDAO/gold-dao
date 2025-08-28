import { useState, useEffect } from "react";
import SelectEnv from "./components/SelectEnv";
import Summary from "./components/summary/Summary";
import RewardsPool from "./components/reward-pools/RewardPools";
import StakePositions from "./components/stake-positions/StakePositions";
import RewardAllocations from "./components/reward-allocations/RewardAllocations";
import ApyHistory from "./components/apy-history/ApyHistory";
// import { getCanister } from "./utils/getCanister";

function App() {
  const [env, setEnv] = useState("production");

  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const envFromUrl = urlParams.get("env");

    if (
      envFromUrl &&
      (envFromUrl === "staging" || envFromUrl === "production")
    ) {
      setEnv(envFromUrl);
    } else {
      const url = new URL(window.location);
      url.searchParams.set("env", "production");
      window.history.replaceState({}, "", url);
      setEnv("production");
    }
  }, []);

  return (
    <div className="xl:container mx-4 xl:mx-auto my-16">
      <div className="flex justify-between items-center mb-8 xl:mb-16">
        <h1 className="text-2xl font-bold">GLDT Stake Dashboard</h1>
        <SelectEnv handleOnChange={(env) => setEnv(env)} className="" />
      </div>
      <Summary env={env} />

      <div className="mt-8 md:mt-16">
        <h3 className="text-lg md:text-xl xl:text-2xl mb-4 md:mb-8">
          Rewards pool
        </h3>
        <RewardsPool env={env} />
      </div>

      <div className="mt-8 md:mt-16">
        <h3 className="text-lg md:text-xl xl:text-2xl mb-4 md:mb-8">
          Stake positions
        </h3>
        <StakePositions env={env} />
      </div>

      <div className="mt-8 md:mt-16">
        <h3 className="text-lg md:text-xl xl:text-2xl mb-4 md:mb-8">
          Reward allocations
        </h3>
        <RewardAllocations env={env} />
      </div>

      <div className="mt-8 md:mt-16">
        <h3 className="text-lg md:text-xl xl:text-2xl mb-4 md:mb-8">
          APY history
        </h3>
        <ApyHistory env={env} />
      </div>
    </div>
  );
}

export default App;
