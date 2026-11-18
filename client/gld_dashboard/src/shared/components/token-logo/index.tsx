import { useState } from "react";
import clsx from "clsx";
import { useQuery } from "@tanstack/react-query";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import { Token } from "@shared/utils/tokens";
import { fetch_ledger_logo } from "@services/ledger/fetch_ledger_token_info";

// Query key of the logo a ledger publishes in its metadata.
export const ledgerLogoQueryKey = (canisterId: string) => [
  "FETCH_LEDGER_LOGO",
  canisterId,
];

// Stands in for a token that has no logo: its first two letters in a circle.
const LogoPlaceholder = ({
  symbol,
  className,
}: {
  symbol: string;
  className: string;
}) => (
  <svg
    viewBox="0 0 24 24"
    role="img"
    aria-label={`${symbol} logo`}
    className={clsx(className, "shrink-0")}
  >
    <circle
      cx="12"
      cy="12"
      r="11.5"
      className="fill-surface-secondary stroke-border"
    />
    <text
      x="12"
      y="12"
      textAnchor="middle"
      dominantBaseline="central"
      fontSize="9"
      fontWeight="600"
      className="fill-content/60"
    >
      {symbol.slice(0, 2).toUpperCase()}
    </text>
  </svg>
);

const CustomTokenLogo = ({
  token,
  className,
}: {
  token: Token;
  className: string;
}) => {
  const { unauthenticatedAgent } = useAuth();
  const [isBroken, setIsBroken] = useState(false);

  const logo = useQuery({
    queryKey: ledgerLogoQueryKey(token.canister_id),
    queryFn: () => fetch_ledger_logo(unauthenticatedAgent!, token.canister_id),
    enabled: !!unauthenticatedAgent,
    staleTime: Infinity,
  });

  if (!logo.data || isBroken) {
    return <LogoPlaceholder symbol={token.name} className={className} />;
  }

  return (
    <img
      src={logo.data}
      alt={`${token.name} logo`}
      className={clsx(className, "shrink-0 rounded-full object-cover")}
      onError={() => setIsBroken(true)}
    />
  );
};

// Logo of any wallet token: the bundled image for a built-in token, the image
// from the ledger metadata for a token the user added.
const TokenLogo = ({
  token,
  className = "w-10 h-10",
}: {
  token: Token;
  className?: string;
}) => {
  if (token.is_custom) {
    return <CustomTokenLogo token={token} className={className} />;
  }
  return <Logo name={token.id} className={className} />;
};

export default TokenLogo;
