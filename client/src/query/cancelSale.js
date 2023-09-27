const cancelSale = async (token_id, weight, actor) => {
    const ind = weights.indexOf(weight + 'g');
    const res = await actor[ind]?.sale_batch_nft_origyn([{ end_sale: token_id }]);
    return res;
};
