import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import { GLDT_STAKE_CANISTER_ID } from "@constants";
import useGetTotalStakedAmount from "@earn/hooks/useGetTotalStakedAmount";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const TotalStakedAmount = () => {
  const { unauthenticatedAgent } = useAuth();

  const totalStakedAmount = useGetTotalStakedAmount(
    GLDT_STAKE_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );

  return (
    <div className="border rounded-xl p-4 border-border">
      <div className="flex flex-col items-center xl:items-start gap-1">
        <div className="text-content/60">Total GLDT staked</div>
        <div
          className={`flex items-start gap-2 ${
            !totalStakedAmount.isSuccess ? "animate-pulse" : ""
          }`}
        >
          <div className="rounded-full p-1 bg-background">
            <Logo name="gldt" className="w-10" />
          </div>
          <div>
            <div className="flex items-center gap-1 text-2xl font-semibold">
              <NumberToLocaleString
                value={
                  totalStakedAmount.isSuccess
                    ? totalStakedAmount.data.amount
                    : 0
                }
                decimals={0}
              />
              <div>GLDT</div>
            </div>
            <div className="text-content/60 text-sm">
              $
              <NumberToLocaleString
                value={
                  totalStakedAmount.isSuccess
                    ? totalStakedAmount.data.amount_usd
                    : 0
                }
                decimals={0}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TotalStakedAmount;
