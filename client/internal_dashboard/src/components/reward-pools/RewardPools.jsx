import { encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { AccountIdentifier, SubAccount } from "@dfinity/ledger-icp";
import { hexStringToUint8Array } from "@dfinity/utils";
import useFetchLedgerBalance from "../../hooks/useFetchLedgerBalance";
import { getCanister } from "../../utils/getCanister";
import NumberToLocaleString from "../shared/NumberToLocaleString";
import Card from "../shared/ui/Card";
import { getNeuron } from "../../utils/getNeuron";
import { Principal } from "@dfinity/principal";
import {
  ALLOCATED_REWARDS_SUBACCOUNT,
  PROCESSING_REWARDS_SUBACCOUNT,
  UNALLOCATED_REWARDS_SUBACCOUNT,
} from "../../constants";

const Token = ({ token, canisterId, account, baseLink }) => {
  const balance = useFetchLedgerBalance(canisterId, {
    ledger: token,
    owner: account.owner,
    subaccount: account?.subaccount,
  });
  return (
    <div className="flex justify-between items-center">
      <div className="text-xl md:text-2xl font-light">
        <div className="flex items-baseline gap-1">
          {balance.isSuccess ? (
            <NumberToLocaleString value={balance.data.amount} />
          ) : (
            <div className="animate-pulse">0</div>
          )}
          <div className="text-neutral-900/60 dark:text-neutral-50/60 text-sm">
            {token}
          </div>
        </div>
      </div>
      <Card.Link
        href={`${baseLink}${
          token !== "ICP"
            ? encodeIcrcAccount({
                owner: Principal.fromText(account.owner),
                subaccount: hexStringToUint8Array(account?.subaccount),
              })
            : AccountIdentifier.fromPrincipal({
                principal: Principal.fromText(account.owner),
                subAccount: SubAccount.fromBytes(
                  hexStringToUint8Array(account?.subaccount)
                ),
              }).toHex()
        }`}
      />
    </div>
  );
};

const RewardPools = ({ env }) => {
  const TOKENS = [
    {
      name: "GOLDAO",
      canisterId: getCanister("production").GOLDAO_LEDGER_CANISTER_ID,
      baseLink:
        "https://dashboard.internetcomputer.org/sns/tw2vt-hqaaa-aaaaq-aab6a-cai/account/",
    },
    {
      name: "ICP",
      canisterId: getCanister("production").ICP_LEDGER_CANISTER_ID,
      baseLink: "https://dashboard.internetcomputer.org/account/",
    },
    {
      name: "OGY",
      canisterId: getCanister("production").OGY_LEDGER_CANISTER_ID,
      baseLink:
        "https://dashboard.internetcomputer.org/sns/leu43-oiaaa-aaaaq-aadgq-cai/account/",
    },
  ];

  const NEURON_REWARDS_SOURCE_ACCOUNT = {
    owner: "iyehc-lqaaa-aaaap-ab25a-cai",
    subaccount: getNeuron(env).GLDT_STAKE_SOURCE_NEURON_ID,
  };

  const UNALLOCATED_REWARDS_ACCOUNT = {
    owner: getCanister(env).GLDT_STAKE_CANISTER_ID,
    subaccount: UNALLOCATED_REWARDS_SUBACCOUNT,
  };

  const PROCESSING_REWARDS_ACCOUNT = {
    owner: getCanister(env).GLDT_STAKE_CANISTER_ID,
    subaccount: PROCESSING_REWARDS_SUBACCOUNT,
  };

  const ALLOCATED_REWARDS_ACCOUNT = {
    owner: getCanister(env).GLDT_STAKE_CANISTER_ID,
    subaccount: ALLOCATED_REWARDS_SUBACCOUNT,
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
      <Card>
        <Card.Header className="mb-6">
          <Card.Title>Neuron rewards source</Card.Title>
        </Card.Header>
        <div className="grid grid-cols-1 gap-6">
          {TOKENS.map((token) => (
            <Token
              key={token.name}
              token={token.name}
              canisterId={token.canisterId}
              account={NEURON_REWARDS_SOURCE_ACCOUNT}
              baseLink={token.baseLink}
            />
          ))}
        </div>
      </Card>
      <Card>
        <Card.Header className="mb-6">
          <Card.Title>Unallocated rewards</Card.Title>
        </Card.Header>
        <div className="grid grid-cols-1 gap-6">
          {TOKENS.map((token) => (
            <Token
              key={token.name}
              token={token.name}
              canisterId={token.canisterId}
              account={UNALLOCATED_REWARDS_ACCOUNT}
              baseLink={token.baseLink}
            />
          ))}
        </div>
      </Card>
      <Card>
        <Card.Header className="mb-6">
          <Card.Title>Processing rewards</Card.Title>
        </Card.Header>
        <div className="grid grid-cols-1 gap-6">
          {TOKENS.map((token) => (
            <Token
              key={token.name}
              token={token.name}
              canisterId={token.canisterId}
              account={PROCESSING_REWARDS_ACCOUNT}
              baseLink={token.baseLink}
            />
          ))}
        </div>
      </Card>
      <Card>
        <Card.Header className="mb-6">
          <Card.Title>Allocated rewards</Card.Title>
        </Card.Header>
        <div className="grid grid-cols-1 gap-6">
          {TOKENS.map((token) => (
            <Token
              key={token.name}
              token={token.name}
              canisterId={token.canisterId}
              account={ALLOCATED_REWARDS_ACCOUNT}
              baseLink={token.baseLink}
            />
          ))}
        </div>
      </Card>
    </div>
  );
};

export default RewardPools;
