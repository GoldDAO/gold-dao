import { toast, Bounce } from "react-toastify";
import { p } from "../utils/parsers";
import useActor from "./useActor";
import { useSession } from "./useSession";

const useBalances = () => {
  const [ogy] = useActor("ogy");
  const [icp] = useActor("icp");
  const [ledger] = useActor("ledger");
  const tokens = { ogy, icp, ledger };
  const { principal, isConnected } = useSession();

  const getBalance = async (tokenName = "icp") => {
    if (!isConnected) {
      return 0;
    }
    const token = tokens[tokenName];
    if (!token) return 0;

    try {
      const balance = await token.icrc1_balance_of({ owner: p(principal), subaccount: [] });

      return Number(balance);
    } catch (err) {
      console.log("get balance error");

      return 0;
    }
  };

  return { getBalance };
};

export default useBalances;
