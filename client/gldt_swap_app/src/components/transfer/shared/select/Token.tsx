import { Token as TokenType, useTransfer } from "@context/index";
import { Select } from "@components/ui";

const TOKEN_OPTIONS = [
  { value: "GLDT", icon: "gldt_logo.svg", label: "ICRC-1" },
  { value: "GLD NFT", icon: "gold-bars/1g.svg", label: "NFT" },
  { value: "OGY", icon: "ogy_logo.svg", label: "ICRC-1" },
];

const Token = ({ className }: { className?: string }) => {
  const { setToken, state: transferState } = useTransfer();

  const handleOnChangeToken = (token: TokenType): void => {
    setToken(token);
  };

  const getValue = () => {
    if (transferState.token === TokenType.GLD_NFT) return "GLD NFT";
    else if (transferState.token === TokenType.GLDT) return "GLDT";
    else return "OGY";
  };

  const getToken = (value: string | number) => {
    if (value === "GLD NFT") return TokenType.GLD_NFT;
    else if (value === "GLDT") return TokenType.GLDT;
    else return TokenType.OGY;
  };

  return (
    <div className={className}>
      <Select
        options={TOKEN_OPTIONS}
        value={getValue()}
        handleOnChange={(value) => handleOnChangeToken(getToken(value))}
      />
    </div>
  );
};

export default Token;
