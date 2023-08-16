import { useCanister } from '@connect2ic/react';
import React from 'react';

const NFTsSaleInfos = () => {
  //   const [actor1g] = useCanister('NFT_1G_CANISTER');
  const [actor10g] = useCanister('10g');
  //   const [actor100g] = useCanister('NFT_100G_CANISTER');
  //   const [actor1000g] = useCanister('NFT_1000G_CANISTER');

  const payload = () => {
    return {
      status: '6b9e47542c6573065b84b3e8a9dbd7da8f036b36eb1e9bcf8c5efe0ba3b055f7',
    };
  };

  const handleButton = async () => {
    const res = await actor10g.sale_info_batch_nft_origyn([
      {
        active: [],
      },
    ]);
    console.log('res1', res);
    const sale_id = '641b980a7144674d812926971c8fe419a4165024be2b6d4043dba2f4bf3c9047';
    const res2 = await actor10g.sale_info_batch_nft_origyn([
      {
        status: sale_id,
      },
    ]);
    console.log(res2);
    // const res1g = await actor1g.sale(goldNft1gCart);
    // const res10g = await actor10g.sale_info_batch_nft_origyn(goldNft10gCart);
    // const res100g = await actor100g.sale_info_batch_nft_origyn(goldNft100gCart);
    // const res1000g = await actor1000g.sale_info_batch_nft_origyn(goldNft1000gCart);
    // console.log('res1g', res1g);
    // console.log('res100g', res100g);
    // console.log('res1000g', res1000g);
  };

  return (
    <>
      <button onClick={() => handleButton()}>sale info</button>
    </>
  );
};

export default NFTsSaleInfos;
