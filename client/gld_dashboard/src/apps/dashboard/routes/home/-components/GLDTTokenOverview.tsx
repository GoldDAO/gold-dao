// import { InformationCircleIcon } from "@heroicons/react/20/solid";
import { useTranslation } from "react-i18next";

import useFetchTotalSupply from "@services/ledger/hooks/useFetchTotalSupply";
import useFetchTokenPrice from "@services/icpswap/hooks/useFetchTokenPrice";
import useFetchLockedGLDNFT from "@services/gld_nft/hooks/useFetchLockedGLDNFT";

// import { LoaderSpin, Logo } from "@components/index";
import {
  GLDT_LEDGER_CANISTER_ID_IC,
  ICPSWAP_CANISTER_ID,
  SWAP_CANISTER_ID,
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
} from "@constants";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import { divideBy1e8 } from "@utils/numbers";
import { useAuth } from "@auth/index";

const GLDTTokenOverview = ({ className }: { className?: string }) => {
  const { t } = useTranslation("dashboard/gldt_token_overview");
  const { unauthenticatedAgent } = useAuth();

  const totalSupply = useFetchTotalSupply(
    GLDT_LEDGER_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      ledger: "gldt",
      enabled: !!unauthenticatedAgent,
    }
  );

  const price = useFetchTokenPrice(ICPSWAP_CANISTER_ID, unauthenticatedAgent, {
    tokenSymbol: "GLDT",
    tokenCanisterId: GLDT_LEDGER_CANISTER_ID_IC,
    enabled: !!unauthenticatedAgent,
  });

  const lockedGLDNFT = useFetchLockedGLDNFT(
    GLD_NFT_1G_CANISTER_ID,
    GLD_NFT_10G_CANISTER_ID,
    GLD_NFT_100G_CANISTER_ID,
    GLD_NFT_1000G_CANISTER_ID,
    unauthenticatedAgent,
    {
      owner: SWAP_CANISTER_ID,
      enabled: !!unauthenticatedAgent,
    }
  );

  return (
    <div className={className}>
      <div className="border border-border rounded-xl bg-surface-primary">
        <div className="grid grid-cols-1 lg:grid-cols-2">
          <div className="p-12">
            <div className="text-center lg:text-left mb-6">
              <h4 className="text-2xl font-semibold mb-6">{t("title")}</h4>
              <p>{t("description")}</p>
            </div>
            <div>
              <div className="flex flex-col items-center lg:items-start">
                {/* Locked GLD NFT */}
                <div className="mb-2">
                  {lockedGLDNFT.isSuccess && (
                    <div className="flex gap-3">
                      <div>{t("total_weight_locked_in_canister_in_kg")}</div>
                      <div>{lockedGLDNFT.data} Kg</div>
                    </div>
                  )}
                </div>
                {/* Number of swaps */}
                <div className="mb-2">
                  <div className="flex gap-3">
                    <div>{t("number_of_swaps")}</div>
                    <div>(TODO)</div>
                  </div>
                </div>
                {/* Marketcap GLDT USD */}
                <div className="mb-2">
                  {totalSupply.isSuccess && price.isSuccess && (
                    <div className="flex gap-3">
                      <div>{t("gldt_marketcap")} </div>
                      <div>
                        $
                        <NumberToLocaleString
                          value={divideBy1e8(totalSupply.data) * price.data}
                        />
                      </div>
                    </div>
                  )}
                </div>
              </div>
            </div>
          </div>
          <div className="bg-[#EBE7D6] dark:bg-surface-secondary rounded-b-xl lg:rounded-bl-none lg:rounded-r-xl bg-[url('/src/assets/bg-cover.png')] bg-cover">
            <div className="p-12">
              <video autoPlay loop muted preload="auto" playsInline>
                <source
                  src="https://daolink-gold-dao-website-medias.sos-ch-gva-2.exo.io/GLDNFT2GLDT.webm#t=2.106585"
                  type="video/webm"
                />
                Your browser does not support the video tag :(.
              </video>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default GLDTTokenOverview;
