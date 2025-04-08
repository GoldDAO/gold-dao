import { useSearchParams } from "react-router-dom";

import TokenList from "./token/List";
const List = () => {
  const [searchParams] = useSearchParams();

  return searchParams.get("token") === "nft" ? (
    <div>NFTs tx history</div>
  ) : (
    <TokenList />
  );
};

export default List;
