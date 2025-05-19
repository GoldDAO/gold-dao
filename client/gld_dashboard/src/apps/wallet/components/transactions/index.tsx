import { useSearchParams } from "react-router-dom";
import TokenList from "./TokenList";
import NFTList from "./NFTList";

const Transactions = () => {
  const [searchParams] = useSearchParams();

  return searchParams.get("token") === "nft" ? <NFTList /> : <TokenList />;
};

export default Transactions;
