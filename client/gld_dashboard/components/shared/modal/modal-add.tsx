"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import Link from "next/link";
import { copyContent } from "../../../utils/functions";
import useNeurons from "../../../hooks/useNeurons";
import { useSession } from "../../../hooks/useSession";

export default function ModalAdd({ setNeuronModify }) {
  const [neuronIdToAdd, setNeuronIdToAdd] = useState("");
  const [copyState, setCopyState] = useState(false);
  const { principal } = useSession();

  const { addNeuron, loading, requestSent } = useNeurons({
    neuronId: neuronIdToAdd,
    token: "",
    neuronsToClaim: [],
  });

  useEffect(() => {
    if (requestSent && !loading) setNeuronIdToAdd("");
  }, [requestSent, loading]);

  useEffect(() => {
    if (copyState) {
      setTimeout(() => {
        setCopyState(false);
      }, 2000);
    }
  }, [copyState]);

  const handleAddNeuron = async () => {
    await addNeuron();
    setNeuronModify((prev) => !prev);
  };

  return (
    <div className="flex-col mt-6 mb-32 overflow-y-auto sm:mb-0 flex justify-around items-center">
      <p className="text-left font-normal text-lg w-full flex flex-wrap gap-[36px]">
        <p>
          To successfully add each neuron to the dashboard, please complete the following two steps for every individual neuron:
        </p>
        <span className="flex flex-wrap">
          {"1. Add your principal".split(" ").map((l, i) => (
            <span className="mr-1" key={i * 999}>
              {l}
            </span>
          ))}
          {principal.split("").map((l, i) => (
            <span className="font-bold" key={i * 22}>
              {l}
            </span>
          ))}
          <Image
            src={"/svg/copy-button.svg"}
            alt="copy"
            className="mx-2 cursor-pointer"
            height={15}
            width={15}
            onClick={() => copyContent(principal, setCopyState)}
          />
          <span className="mr-1">as</span>
          <span className="font-bold mr-1">a</span>
          <span className="font-bold mr-1">HotKey</span>
          <span className="mr-1">to</span>
          <span className="mr-1">your</span>
          <span className="font-bold mr-1">Gold</span>
          <span className="font-bold mr-1">DAO</span>
          <span className="font-bold mr-1">neuron</span>
          <span className="mr-1">which</span>
          <span className="mr-1">you</span>
          <span className="mr-1">wish</span>
          <span className="mr-1">to</span>
          <span className="mr-1">include</span>
          <span className="mr-1">in</span>
          <span className="mr-1">this</span>
          <span className="mr-1">dashbaord.</span>
          <span className="mr-1">To</span>
          <span className="mr-1">do</span>
          <span className="mr-1">this,</span>
          <span className="mr-1">Open</span>
          <span className="mr-1">your</span>
          <Link
            href={"https://nns.ic0.app/neurons/?u=tw2vt-hqaaa-aaaaq-aab6a-cai"}
            target="_blank"
            rel="noreferrer noopener"
            className="underline mr-1"
          >
            NNS app
          </Link>
          <span className="mr-1">and</span>
          <span className="mr-1">click</span>
          <span className="mr-1">into</span>
          <span className="mr-1">each</span>
          <span className="mr-1">Neuron.</span>
        </span>
        {copyState && (
          <div className="text-green-600">
            <p>copied</p>
          </div>
        )}
      </p>
      <div className="mt-6 w-full flex flex-col justify-between items-center">
        <p className="text-left w-full font-normal text-lg mb-2">
          2. Enter your <span className="font-bold">Gold DAO neuron ID</span> here:
        </p>
        <label className="input input-bordered flex items-center gap-2 w-full rounded-md bg-white">
          <input
            type="text"
            className="grow"
            placeholder="Neuron ID"
            value={neuronIdToAdd}
            onChange={(e) => setNeuronIdToAdd(e.target.value)}
            disabled={loading}
          />{" "}
          {/* <Image src="svg/qr.svg" alt="hola" height={20} width={20} className="cursor-pointer" /> */}
        </label>
      </div>
      <button
        className={`px-10 mt-6 py-4 rounded-3xl bg-[#D3B871] text-white text-md font-bold flex items-center justify-center ${loading ? "opacity-35 gap-2" : ""} disabled:opacity-35`}
        onClick={handleAddNeuron}
        disabled={loading || !neuronIdToAdd}
      >
        {loading && <span className="loading loading-spinner"></span>}
        Confirm
      </button>
    </div>
  );
}
