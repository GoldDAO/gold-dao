import MainButton from "../UI/button/Buttons";
import { gldNftCanisters } from "@/services/agents";
import { useAllCanisters } from "@/components/commands/hooks/useAllCanisters";

const weights = Object.keys(gldNftCanisters);

const unlistHandler = async (token_id, weight, actors) => {
    const ind = weights.indexOf(weight + 'g');
    const res = await actors[ind]?.sale_batch_nft_origyn([{ end_sale: token_id }]);
    console.log(res);
};


export const CancelsaleButton = ({ token_id, weight, }) => {
    const actors = useAllCanisters()
    return (
        <MainButton label="Cancel Sale" actors={actors} action={() => unlistHandler(token_id, weight, actors)} />
    )
}
