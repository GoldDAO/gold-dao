import { useEffect, useState } from "react";
import { getActor } from "@amerej/artemis-react";
import { canisters } from "@providers/Auth";
import { GetPoolsForTokenResponse } from "@canisters/icp_swap/interfaces";

export const useGetGLDTPrice = () => {
  const [GLDTPrice, setGLDTPrice] = useState(0);
  const { canisterId, idlFactory } = canisters["icp_swap"];

  const gldGovPrice = async () => {
    const icpSwap = await getActor(canisterId, idlFactory, {
      isAnon: true,
    });
    try {
      const gldt_ledger_id = "tyyy3-4aaaa-aaaaq-aab7a-cai"; // todo set to GLDT_LEDGER_CANISTER_ID when icp swap has pair . using gldgov for now
      const pools = (await icpSwap.getPoolsForToken(gldt_ledger_id)) as Awaited<
        GetPoolsForTokenResponse[]
      >;

      const pool = pools?.find(
        (po) => po?.token0Symbol === "GLDGov" && po?.token1Symbol === "ICP"
      );
      if (pool) return pool.token0Price;
      const pool2 = pools?.find(
        (po) => po?.token1Symbol === "GLDGov" && po?.token0Symbol === "ICP"
      );
      return pool2?.token1Price || 0.05;
    } catch (err) {
      console.log("GLDT price error:", err);
      return 0.05;
    }
  };

  useEffect(() => {
    const fetchData = async () => {
      const price = await gldGovPrice();
      setGLDTPrice(price);
    };
    fetchData().catch(console.error);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return {
    GLDTPrice,
  };
};
