import { useAtomValue } from "jotai";
import { useSearchParams } from "react-router-dom";

import { GLDT_VALUE_1G_NFT } from "@constants";
import { useAuth } from "@auth/index";

import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";

import useUserNFTMetrics from "@hooks/useUserNFTMetrics";

import { Logo } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";

import { TokenSelectedAtom } from "./balance.atoms";

const Token = ({ className }: { className?: string }) => {
  const { principalId, authenticatedAgent, isConnected } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);
  const { id, name, label } = token;

  const balance = useFetchUserBalance(token.canisterId, authenticatedAgent, {
    ledger: id,
    owner: principalId,
    enabled: !!authenticatedAgent && !!isConnected,
  });

  const decimals = useFetchDecimals(token.canisterId, authenticatedAgent, {
    ledger: id,
    enabled: !!authenticatedAgent && !!isConnected,
  });

  const renderTokenUserBalance = () => {
    return (
      <div>
        {balance.isSuccess && decimals.isSuccess ? (
          <div className="text-2xl lg:text-4xl font-semibold flex items-center gap-2">
            <TokenValueToLocaleString
              value={balance.data}
              tokenDecimals={decimals.data}
            />
            <div className="text-content/60 font-normal">{name}</div>
          </div>
        ) : (
          <div>Loading...</div>
        )}
      </div>
    );
  };

  const renderTokenGLDT = () => {
    return (
      <div className="flex flex-col items-center gap-2">
        {renderTokenUserBalance()}
        <div>
          {balance.isSuccess && decimals.isSuccess ? (
            <div>
              <TokenValueToLocaleString
                value={balance.data / BigInt(GLDT_VALUE_1G_NFT)}
                tokenDecimals={decimals.data}
              />{" "}
              grams of Gold ($todo)
            </div>
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    );
  };

  const renderToken = () => {
    return (
      <div className="flex flex-col items-center gap-2">
        {renderTokenUserBalance()}
        <div>($todo)</div>
      </div>
    );
  };

  const renderTokenHeaderContent = () => {
    switch (id) {
      case "gldt":
        return renderTokenGLDT();
      default:
        return renderToken();
    }
  };

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex flex-col gap-2 items-center">
          <div className="flex items-center gap-2">
            <Logo name={id} className="h-10 w-10" />
            <div>
              <div>{name}</div>
              <div className="text-content/60 text-sm">{label}</div>
            </div>
          </div>
          <div className="text-sm text-content/60">1 {name} ≈ $todo</div>
        </div>
        <div className="py-8 lg:py-12">{renderTokenHeaderContent()}</div>
      </div>
    </div>
  );
};

const NFT = ({ className }: { className?: string }) => {
  // const { principalId, authenticatedAgent, isConnected } = useAuth();

  const { data: nfts, isSuccess: isSuccessFetchUserNFTs } = useUserNFTMetrics();

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex flex-col gap-2 items-center">
          <div className="flex items-center gap-2">
            <Logo name="gld_nft" className="h-10 w-10" />
            <div>
              <div>GLD NFT</div>
              <div className="text-content/60 text-sm">GLD NFT</div>
            </div>
          </div>
          {/* <div className="text-sm text-content/60">1 {name} ≈ $(todo)</div> */}
        </div>
        <div className="py-8 lg:py-12">
          <div className="flex flex-col items-center gap-2">
            <div className="text-2xl lg:text-4xl font-semibold">
              {isSuccessFetchUserNFTs ? (
                <div className="flex items-center gap-2">
                  {nfts.totalCount}
                  <div className="text-content/60 font-normal">NFTs</div>
                </div>
              ) : (
                <div>Loading...</div>
              )}
            </div>
            {isSuccessFetchUserNFTs ? (
              <div>{nfts.totalGrams} grams of Gold ($todo)</div>
            ) : (
              <div>Loading...</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

const BalanceHeader = ({ className }: { className?: string }) => {
  const [searchParams] = useSearchParams();

  return (
    <div className={className}>
      {searchParams.get("token") === "nft" ? <NFT /> : <Token />}
    </div>
  );
};

export default BalanceHeader;
