import { Token as TokenType, useTransfer } from "@context/index";
import { SelectToken } from "@components/ui/select/index";
import { LogoGLDT } from "@components/shared/logos";

const TOKEN_OPTIONS = [
  { value: "GLDT", icon: <LogoGLDT className="w-4 h-4" />, label: "ICRC-2" },
  {
    value: "GLD NFT",
    icon: <img src="/nft_logo.svg" className="w-4 h-4" alt="Gold bar logo" />,
    label: "NFT",
  },
  {
    value: "OGY",
    icon: <img src="/ogy_logo.svg" className="w-4 h-4" alt="Origyn logo" />,
    label: "ICRC-2",
  },
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
      <SelectToken
        options={TOKEN_OPTIONS}
        value={getValue()}
        handleOnChange={(value) => handleOnChangeToken(getToken(value))}
      />
    </div>
  );
};

export default Token;
