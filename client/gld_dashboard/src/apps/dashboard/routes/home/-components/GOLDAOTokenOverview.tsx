import { InformationCircleIcon } from "@heroicons/react/20/solid";
import { useTranslation } from "react-i18next";

import { LoaderSpin, Logo } from "@components/index";

import useFetchTotalSupply from "@services/ledger/hooks/useFetchTotalSupply";
import useFetchTokenPrice from "@services/icpswap/hooks/useFetchTokenPrice";

import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import E8sToLocaleString from "@components/numbers/E8sToLocaleString";

import { GOLDAO_LEDGER_CANISTER_ID_IC, ICPSWAP_CANISTER_ID } from "@constants";
import { divideBy1e8 } from "@utils/numbers";
import { useAuth } from "@auth/index";

const GOLDAOTokenOverview = ({ className }: { className?: string }) => {
  const { t } = useTranslation("dashboard/gldgov_token_overview");
  const { unauthenticatedAgent } = useAuth();

  const totalSupply = useFetchTotalSupply(
    GOLDAO_LEDGER_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      ledger: "gldgov",
      enabled: !!unauthenticatedAgent,
    }
  );

  const price = useFetchTokenPrice(ICPSWAP_CANISTER_ID, unauthenticatedAgent, {
    tokenSymbol: "GOLDAO",
    tokenCanisterId: GOLDAO_LEDGER_CANISTER_ID_IC,
    enabled: !!unauthenticatedAgent,
  });

  return (
    <div className={className}>
      <div className="grid grid-cols-1 xl:grid-cols-3 gap-6 justify-between">
        <div className="w-full">
          <div className="border border-border rounded-xl bg-surface-primary p-6 xl:p-8">
            <div className="flex items-center gap-3 mb-4 justify-center xl:justify-start">
              <div className="font-light text-content/60">
                {t("total_gldgov_supply.title")}
              </div>
              <InformationCircleIcon
                className="size-5 text-accent/60"
                data-tooltip-id="tooltip"
                data-tooltip-content={t("total_gldgov_supply.tooltip")}
              />
            </div>

            {totalSupply.isSuccess && (
              <div className="flex items-center justify-center xl:justify-start gap-4">
                <Logo className="flex-none h-8" name="gldgov" />
                <div className="font-semibold text-4xl">
                  <E8sToLocaleString value={totalSupply.data} />
                </div>
              </div>
            )}
            {(totalSupply.isLoading || totalSupply.isError) && (
              <div className="flex justify-center">
                <LoaderSpin />
              </div>
            )}
          </div>
        </div>
        <div className="w-full">
          <div className="border border-border rounded-xl bg-surface-primary p-6 xl:p-8">
            <div className="flex items-center gap-3 mb-4 justify-center xl:justify-start">
              <div className="font-light text-content/60">
                {t("gldgov_price.title")}
              </div>
              <InformationCircleIcon
                className="size-5 text-accent/60"
                data-tooltip-id="tooltip"
                data-tooltip-content={t("gldgov_price.tooltip")}
              />
            </div>

            {price.isSuccess && (
              <div className="flex items-center justify-center xl:justify-start gap-4">
                <Logo className="flex-none h-8" name="gldgov" />
                <div className="font-semibold text-4xl">
                  $<NumberToLocaleString value={price.data} />
                </div>
              </div>
            )}
            {(price.isLoading || price.isError) && (
              <div className="flex justify-center">
                <LoaderSpin />
              </div>
            )}
          </div>
        </div>
        <div className="w-full">
          <div className="border border-border rounded-xl bg-surface-primary p-6 xl:p-8">
            <div className="flex items-center gap-3 mb-4 justify-center xl:justify-start">
              <div className="font-light text-content/60">
                {t("gldgov_marketcap.title")}
              </div>
              <InformationCircleIcon
                className="size-5 text-accent/60"
                data-tooltip-id="tooltip"
                data-tooltip-content={t("gldgov_marketcap.tooltip")}
              />
            </div>

            {totalSupply.isSuccess && price.isSuccess && (
              <div className="flex items-center justify-center xl:justify-start gap-4">
                <Logo className="flex-none h-8" name="gldgov" />
                <div className="font-semibold text-4xl">
                  $
                  <NumberToLocaleString
                    value={divideBy1e8(totalSupply.data) * price.data}
                  />
                </div>
              </div>
            )}
            {(totalSupply.isLoading ||
              totalSupply.isError ||
              price.isLoading ||
              price.isError) && (
              <div className="flex justify-center">
                <LoaderSpin />
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default GOLDAOTokenOverview;
