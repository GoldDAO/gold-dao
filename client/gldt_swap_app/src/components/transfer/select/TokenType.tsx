import { Token, useTransfer } from "@context/index";
import { Select } from "@components/ui";

const TOKEN_TYPE_OPTIONS = [
  { value: "GLDT", icon: "vite.svg", label: "ICRC-1" },
  { value: "GLD NFT", icon: "vite.svg", label: "NFT" },
  { value: "OGY", icon: "vite.svg", label: "ICRC-1" },
];

const TokenType = ({ className }: { className: string }) => {
  const { setToken, state: transferState } = useTransfer();

  const handleOnChangeToken = (token: Token): void => {
    setToken(token);
  };

  const getValue = () => {
    if (transferState.token === Token.GLD_NFT) return "GLD NFT";
    else if (transferState.token === Token.GLDT) return "GLDT";
    else return "OGY";
  };

  const getToken = (value: string | number) => {
    if (value === "GLD NFT") return Token.GLD_NFT;
    else if (value === "GLDT") return Token.GLDT;
    else return Token.OGY;
  };

  return (
    <div className={className}>
      <Select
        options={TOKEN_TYPE_OPTIONS}
        value={getValue()}
        handleOnChange={(value) => handleOnChangeToken(getToken(value))}
      />
    </div>
  );
};

export default TokenType;
