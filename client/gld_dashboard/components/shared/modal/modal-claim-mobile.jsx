'use client';

import Image from 'next/image';
import { elapsedTime } from '../../../utils/functions';

export default function ModalClaimMobile({ item, setClaimState }) {
  if (!item) return;

  const { votingPower } = item;

  // eslint-disable-next-line consistent-return
  return (
    <>
      <div className=" flex-col mt-6 width-[90%] h-full overflow-y-auto mb-36 flex justify-around items-center">
        <div className="w-full flex justidy-start py-5">
          <p className="text-4xl font-medium">
            <span className="font-bold">
              {item.staked_maturity_e8s_equivalent[0]
                ? Number(item.staked_maturity_e8s_equivalent)
                : 0}
            </span>{' '}
            GOLDAO
          </p>
        </div>
        <div className="w-full flex flex-col justify-center gap-5">
          <div className="w-full flex  justify-center gap-5">
            <button
              className={`z-10 text-white w-full font-bold py-2 px-8 rounded-full sm:hidden flex  gap-2 items-center  justify-center text-sm ${item?.icpRewards > 0 ? 'bg-black' : 'bg-black opacity-50 cursor-not-allowed'}`}
              disabled={item?.icpRewards <= 0}
              onClick={() => {
                setClaimState({
                  name: item.id,
                  amount: item.icpRewards,
                  claim: 'ICP',
                  ...item,
                });
                document.getElementById('my_modal_confirm').showModal();
              }}
            >
              Claim{' '}
              <span className="text-3xl">{(item?.icpRewards || 0) / 10e7}</span>
              <Image
                className="h-4 w-4"
                alt="dfinity"
                src="png/dfinity.png"
                width={13}
                height={13}
              />
            </button>
            <Image src={'svg/aroundarrow.svg'} alt="aroundarrow" height={30} width={30} />
          </div>
          <div className="w-full flex justify-center gap-5">
            <button
              className={`z-10 text-white w-full font-bold py-2 px-8 rounded-full sm:hidden flex justify-center gap-2 items-center text-sm ${item?.ledgerRewards > 0 ? 'bg-black' : 'bg-black opacity-50 cursor-not-allowed'}`}
              disabled={item?.ledgerRewards <= 0}
              onClick={() => {
                setClaimState({
                  name: item.id,
                  amount: item.ledgerRewards,
                  claim: 'GOLDAO',
                  ...item,
                });
                document.getElementById('my_modal_confirm').showModal();
              }}
            >
              Claim{' '}
              <span className="text-3xl">
                {(item?.ledgerRewards || 0) / 10e7}
              </span>
              <Image
                className="h-4 w-4"
                alt="GOLDAO token"
                src="svg/g-logo.svg"
                width={13}
                height={13}
              />
            </button>
            <Image src={'svg/aroundarrow.svg'} alt="aroundarrow" height={30} width={30} />
          </div>
          <div className="flex justify-start gap-5">
            <button
              className={`z-10 text-white w-full font-bold py-2 px-8 rounded-full sm:hidden flex gap-2 items-center justify-center text-sm ${item?.ogyRewards > 0 ? 'bg-black' : 'bg-black opacity-50 cursor-not-allowed'}`}
              disabled={item?.ogyRewards <= 0}
              onClick={() => {
                setClaimState({
                  name: item.id,
                  amount: item.ogyRewards,
                  claim: 'OGY',
                  ...item,
                });
                document.getElementById('my_modal_confirm').showModal();
              }}
            >
              Claim{' '}
              <span className="text-3xl">{(item?.ogyRewards || 0) / 10e7}</span>
              <Image className="h-4 w-4" src="ogy.png" alt="origyn" width={13} height={13} />
            </button>
            <Image src={'svg/aroundarrow.svg'} alt="aroundarrow" height={30} width={30} />
          </div>
        </div>
        <div className="w-full grid grid-cols-2 gap-5 my-10 border-t-2 pt-5">
          <div>
            <div className="text-[#D3B871] font-medium flex  items-center gap-2 text-sm">
              State
              <Image
                className="h-4 w-4"
                alt="GOLDAO governance token"
                src="svg/info.svg"
                width={13}
                height={13}
              />
            </div>
            <div>{item.dissolving}</div>
          </div>
          <div>
            <div className="text-[#D3B871] font-medium flex  items-center gap-2 text-sm">
              Voting Power
              <Image
                className="h-4 w-4"
                alt="GOLDAO token"
                src="svg/info.svg"
                width={13}
                height={13}
              />
            </div>
            <div>{votingPower}</div>
          </div>
        </div>
        <div className="w-full grid grid-cols-2 gap-5  border-t-2 pt-5">
          <div>
            <div className="text-[#D3B871] font-medium flex  items-center gap-2 text-sm">
              Dissolve Delay
              <Image
                className="h-4 w-4"
                alt="GOLDAO token"
                src="svg/info.svg"
                width={13}
                height={13}
              />
            </div>
            <div>{elapsedTime(item.dissolveDelay)}</div>
          </div>
          <div>
            <div className="text-[#D3B871] font-medium flex  items-center gap-2 text-sm">
              Age
              <Image
                className="h-4 w-4"
                alt="GOLDAO token"
                src="svg/info.svg"
                width={13}
                height={13}
              />
            </div>
            {/* [?] if neuron is dissolving the age is considered 0 */}
            <div>{elapsedTime(item?.age) || 0}</div>
          </div>
        </div>
      </div>
    </>
  );
}
