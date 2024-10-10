import { useEffect, useState } from "react";
import { GetPoolsForTokenResponse } from "@canisters/icp_swap/interfaces";

import { useAuth } from "@auth/index";

export const useGetGLDTPrice = () => {
  const { createActor } = useAuth();
  const [GLDTPrice, setGLDTPrice] = useState(0);

  const gldGovPrice = async () => {
    const actor = createActor("icp_swap");
    try {
      const gldt_ledger_id = "tyyy3-4aaaa-aaaaq-aab7a-cai"; // todo set to GLDT_LEDGER_CANISTER_ID when icp swap has pair . using gldgov for now
      const pools = (await actor.getPoolsForToken(gldt_ledger_id)) as Awaited<
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
