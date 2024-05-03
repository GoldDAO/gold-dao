import { Bounce, toast } from 'react-toastify';
import { useEffect, useState } from 'react';
import Image from 'next/image';
import { Tooltip } from '@nextui-org/react';
import { CopyIcon, DoubleArrowRefresh, RedCross } from '../../../utils/svgs';
import {
  copyContent,
  elapsedTime,
  neuronState,
  truncateNeuronId,
  uint8ArrayToHexString,
} from '../../../utils/functions';
import Modal from '../modal/modal';
import ModalAdd from '../modal/modal-add.tsx';
import ModalClaimAll from '../modal/modal-claim-all';
import ModalClaimMobile from '../modal/modal-claim-mobile';
import ModalConfirm from '../modal/modal-confirm';
import ModalDelete from '../modal/modal-delete';
import useNeurons from '../../../hooks/useNeurons';

export default function RewardsNeurons({ setIcp, setGold }) {
  const [claimState, setClaimState] = useState(null);
  const [disableClaimAll, setDisableClaimAll] = useState(true);
  const [neuronAmountsToClaim, setNeuronAmountsToClaim] = useState({});
  const [reloadPrincipal, setReloadPrincipal] = useState(false);
  const [selectedNeuronId, setSelectedNeuronId] = useState(false);
  const [userNeurons, setUserNeurons] = useState([]);
  const [neuronModify, setNeuronModify] = useState([]);
  const [hovered, setHovered] = useState({ disabled: false });
  const [copyState, setCopyState] = useState(false);

  const handleHover = ({ disabled, index }) => {
    setHovered({ ...hovered, disabled, index });
  };

  const { getNeuronsByOwner, loading, neuronError } = useNeurons({
    neuronId: '',
    token: '',
  });

  const getNeurons = async () => {
    const response = await getNeuronsByOwner();
    if (response) {
      setUserNeurons(response);
      const amountsToClaim = response.reduce(
        (acc, curr) => {
          acc.icpAmount += curr.icpRewards;
          acc.ledgerAmount += curr.ledgerRewards;
          return acc;
        },
        { icpAmount: 0, ledgerAmount: 0 },
      );
      setNeuronAmountsToClaim({ ...amountsToClaim, userNeurons: response });
      if (!amountsToClaim.icpAmount && !amountsToClaim.ledgerAmount) {
        setDisableClaimAll(true);
      } else {
        setDisableClaimAll(false);
      }
    }
  };

  useEffect(() => {
    getNeurons();
  }, [neuronModify]);

  const handleReloadClick = () => {
    setReloadPrincipal(true);
    getNeurons();
    setTimeout(() => {
      setReloadPrincipal(false);
    }, 2000);
  };

  useEffect(() => {
    if (copyState) {
      toast.success('Copied', {
        position: 'top-right',
        autoClose: 2000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      setCopyState(false);
    }
  }, [copyState]);

  return (
    <>
      <section className="flex flex-col justify-start  my-0 sm:mx-2  mb-10 relative bg-SoftGrey mt-2 rounded-[2rem] border-[#C6C6C6] border">
        <div className="w-full p-6 flex justify-between items-center h-20 border-[#C6C6C6] border-b text-xs">
          <div className="flex gap-2 font-bold">
            My GLDGov neurons
            <div
              className={`hidden sm:flex rounded-full justify-center items-center cursor-pointer ${reloadPrincipal ? 'animate-spin' : ''}`}
              onClick={handleReloadClick}
            >
              <Image src={'/svg/reload.svg'} alt="" height={20} width={20} />
            </div>
          </div>
          <button
            onClick={() => {
              document.getElementById('my_modal_claim_desk').showModal();
            }}
            className={`px-6 py-2 rounded-3xl bg-[#D3B871] text-white sm:text-sm ${disableClaimAll ? 'opacity-35 cursor-not-allowed' : ''}`}
            disabled={disableClaimAll}
          >
            Claim all
          </button>
        </div>
        {loading ? (
          <article className="collapse w-full border-[1px] rounded-none flex justify-center">
            <section className="collapse-title flex gap-2 justify-center items-center h-20">
              <span className="loading loading-spinner"></span>
              Loading your neurons...
            </section>
          </article>
        ) : (
          userNeurons.map((item) => {
            const dissolveState = item.dissolve_state[0];
            const { votingPower } = item;
            return (
              <article
                key={item.id}
                className="collapse sm:collapse-arrow w-full border-1 rounded-none"
              >
                <input type="checkbox" className="checkboxleft hidden sm:block" />
                <section className="collapse-title flex sm:grid sm:grid-cols-[1.5fr,1fr,1fr,1fr,1fr,1fr] sm:gap-x-4 justify-between items-center h-20 w-full px-2 sm:px-8">
                  <p
                    className="text-lg font-medium hidden sm:flex items-center gap-x-2 z-20"
                    onClick={() => copyContent(uint8ArrayToHexString(item.id), setCopyState)}
                  >
                    {truncateNeuronId(item.id)}
                    {CopyIcon}
                  </p>
                  <p
                    className="text-xs font-medium sm:hidden"
                    onClick={() => copyContent(uint8ArrayToHexString(item.id), setCopyState)}
                  >
                    {truncateNeuronId(uint8ArrayToHexString(item.id), 4, 4)}
                  </p>
                  <p className="text-xs sm:text-lg font-medium">
                    <span className="font-bold">
                      {item.cached_neuron_stake_e8s
                        ? Number(item.cached_neuron_stake_e8s) / 10e7
                        : 0}
                    </span>{' '}
                    GLDGov
                  </p>
                  <button
                    onClick={() => {
                      document.getElementById('my_modal_confirm').showModal();
                      setClaimState({
                        name: item.id,
                        amount: item.icpRewards,
                        claim: 'ICP',
                        ...item,
                      });
                    }}
                    className={`z-10 text-white min-w-[100px] max-w-[200px] font-bold py-2 px-8 rounded-full hidden sm:flex gap-2 items-center justify-center text-sm ${item.icpRewards > 0 ? 'bg-black' : 'bg-black opacity-50 cursor-not-allowed'}`}
                    disabled={item.icpRewards <= 0}
                  >
                    Claim {item.icpRewards / 1e8 || 0}
                    <Image
                      className="h-4 w-4"
                      alt="dfinity"
                      src="png/dfinity.png"
                      width={13}
                      height={13}
                    />
                  </button>
                  <button
                    onClick={() => {
                      document.getElementById('my_modal_confirm').showModal();
                      setClaimState({
                        name: item.id,
                        amount: item.ledgerRewards,
                        claim: 'GLDGov',
                        ...item,
                      });
                    }}
                    className={`z-10 text-white min-w-[100px] max-w-[200px] font-bold py-2 px-8 rounded-full hidden sm:flex  gap-2 items-center justify-center text-sm ${item.ledgerRewards > 0 ? 'bg-black' : 'bg-black opacity-50 cursor-not-allowed'}`}
                    disabled={item.ledgerRewards <= 0}
                  >
                    Claim {item.ledgerRewards / 1e8 || 0}
                    <Image
                      className="h-4 w-4"
                      alt="gldgov governance token"
                      src="svg/g-logo.svg"
                      width={13}
                      height={13}
                    />
                  </button>
                  <button
                    onClick={() => {
                      document.getElementById('my_modal_confirm').showModal();
                      // setClaimState({ name: item.id, amount: item.ogyRewards,
                      // claim: "OGY", ...item });
                    }}
                    className={
                      'z-10 text-white min-w-[160px] max-w-[200px] font-bold py-2 px-8 rounded-full hidden sm:flex gap-2 items-center justify-center text-sm bg-black opacity-50 cursor-not-allowed'
                    }
                    disabled={true}
                  >
                    Claim 0
                    <Image className="h-4 w-4" src="ogy.png" alt="origyn" width={13} height={13} />
                  </button>
                  <button
                    onClick={() => {
                      document.getElementById('my_modal_claim').showModal();
                      setClaimState(item);
                    }}
                    className={`z-10 text-white min-w-[100px] bg-black max-w-[200px] font-bold py-2 px-4 rounded-full sm:hidden flex gap-2 items-center text-sm justify-center ${disableClaimAll ? 'opacity-35 cursor-not-allowed' : ''}`}
                    disabled={disableClaimAll}
                  >
                    Claim
                  </button>
                  <button className="z-10 flex items-center justify-between w-1/6">
                    <div className="sm:hidden">
                      {loading && DoubleArrowRefresh}
                      {neuronError[item.id] && RedCross}
                    </div>
                    <Image
                      src="svg/trash.svg"
                      alt="trash"
                      height={20}
                      width={20}
                      className="ml-2"
                      onClick={() => {
                        setSelectedNeuronId(item.id);
                        document.getElementById('my_modal_delete').showModal();
                      }}
                    />
                  </button>
                </section>
                <section className="collapse-content w-[95%] grid grid-cols-4 grid-rows-2 px-2 sm:px-8">
                  <div className="text-[#D3B871] font-medium flex  items-center gap-2 text-sm">
                    {' '}
                    State
                    <div
                      className="cursor-pointer z-30"
                    >
                      <Tooltip
                        content='The state of the neuron.'
                        classNames={{
                          base: [
                            'max-w-[400px]',
                          ],
                          content: [
                            'py-2 px-4 shadow-xl',
                            'text-white bg-black',
                          ],
                        }}
                      >
                        <Image src={'/svg/info.svg'} alt="info" height={20} width={20} />
                      </Tooltip>
                    </div>
                  </div>
                  <div className="text-[#D3B871] font-medium flex  items-center gap-2 text-sm">
                    {' '}
                    Voting Power
                    <div
                      className="cursor-pointer z-30"
                    >
                      <Tooltip
                        content='The voting power of the neuron.'
                        classNames={{
                          base: [
                            'max-w-[400px]',
                          ],
                          content: [
                            'py-2 px-4 shadow-xl',
                            'text-white bg-black',
                          ],
                        }}
                      >
                        <Image src={'/svg/info.svg'} alt="info" height={20} width={20} />
                      </Tooltip>
                    </div>
                  </div>
                  <div className="text-[#D3B871] font-medium flex  items-center gap-2 text-sm">
                    {' '}
                    Dissolve Delay
                    <div
                      className="hidden sm:flex"
                      onMouseEnter={() => handleHover({ disabled: true, index: 1 })}
                      onMouseLeave={() => setHovered({ disabled: false, index: null })}
                    >
                      <Tooltip
                        content='The minimum time period over which the neuron owner locks up their staked
                        GLDGov tokens. This determines how long it will take to dissolve if the
                        neuron is placed into the Dissolving state. Once a neuron has been placed
                        into the Dissolving state, its dissolve delay falls over the passage of
                        time, rather like a kitchen timer, until either it is stopped or it reaches
                        zero. When it reaches zero and enters the Dissolved state, its owner can
                        perform a final disburse action to unlock the balance of GLDGov tokens. The
                        dissolve delay can be configured up to a maximum of 2 years, and must be 91
                        days or greater for a neuron to be able to vote and earn voting rewards.'
                        classNames={{
                          base: [
                            'max-w-[400px]',
                          ],
                          content: [
                            'py-2 px-4 shadow-xl',
                            'text-white bg-black',
                          ],
                        }}
                      >
                        <Image
                          src={'/svg/info.svg'}
                          className="relative"
                          alt="info"
                          height={20}
                          width={20}
                        />

                      </Tooltip>
                    </div>
                  </div>
                  <div className="text-[#D3B871] font-medium flex items-center gap-2 text-sm">
                    Age
                    <div
                      className="hidden sm:flex"
                      onMouseEnter={() => handleHover({ disabled: true, index: 2 })}
                      onMouseLeave={() => setHovered({ disabled: false, index: null })}
                    >
                      <Tooltip
                        content='The period of time that has elapsed since the neuron was created or last
                        entered the Not Dissolving state. While dissolving, a neuron’s age is
                        considered zero. Increasing the stake of a neuron will decrease its age. For
                        example, if the stake is doubled, the age will be halved. Splitting a neuron
                        creates a child neuron that inherits the age of its parent.'
                        classNames={{
                          base: [
                            'max-w-[400px]',
                          ],
                          content: [
                            'py-2 px-4 shadow-xl',
                            'text-white bg-black',
                          ],
                        }}
                      >
                        <Image
                          src={'/svg/info.svg'}
                          className="relative"
                          alt="info"
                          height={20}
                          width={20}
                        />
                      </Tooltip>
                    </div>
                  </div>
                  <div className="font-bold text-sm">{neuronState(dissolveState)}</div>
                  <div className="font-bold text-sm">{votingPower}</div>
                  <div className="font-bold text-sm">{elapsedTime(item.dissolveDelay)}</div>
                  <div className="font-bold text-sm">
                    {/* [?] {dissolveState.DissolveDelaySeconds ? elapsedTime(item.age) : '0'} */}
                    {elapsedTime(item?.age) || 0}
                  </div>
                </section>
              </article>
            );
          })
        )}
        <footer
          onClick={() => document.getElementById('my_modal_add').showModal()}
          className="w-[95%] my-0 mx-auto flex gap-8 text-[#C6C6C6] justify-start items-center h-20 border-[#C6C6C6] cursor-pointer "
        >
          <Image src={'/svg/plus.svg'} alt="plus" height={40} width={40} />
          Add Neuron
        </footer>
      </section>
      <Modal title="Remove neuron" idModal="my_modal_delete">
        <ModalDelete neuronId={selectedNeuronId} setNeuronModify={setNeuronModify} />
      </Modal>
      <Modal title="Confirm claim" idModal="my_modal_claim">
        <ModalClaimMobile item={claimState} setClaimState={setClaimState} />
      </Modal>
      <Modal title="Confirm claim" idModal="my_modal_confirm">
        <ModalConfirm
          {...claimState}
          setNeuronModify={setNeuronModify}
          setGold={setGold}
          setIcp={setIcp}
        />
      </Modal>
      <Modal title="Add Neuron" idModal="my_modal_add">
        <ModalAdd setNeuronModify={setNeuronModify} />
      </Modal>
      <Modal title="Confirm claim" idModal="my_modal_claim_desk">
        <ModalClaimAll
          neuronAmountsToClaim={neuronAmountsToClaim}
          setNeuronModify={setNeuronModify}
          setGold={setGold}
          setIcp={setIcp}
        />
      </Modal>
    </>
  );
}
