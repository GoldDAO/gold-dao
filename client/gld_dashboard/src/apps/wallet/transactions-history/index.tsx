import { useSearchParams } from "react-router-dom";

import TokenList from "./TokenList";
import NFTList from "./NFTList";

const List = () => {
  const [searchParams] = useSearchParams();

  return searchParams.get("token") === "nft" ? <NFTList /> : <TokenList />;
};

export default List;
