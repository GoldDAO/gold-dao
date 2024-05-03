import { parseTimestamp } from "../utils/parsers";

const URI =
  "https://icrc-api.internetcomputer.org/api/v1/ledgers/tyyy3-4aaaa-aaaaq-aab7a-cai/accounts/tr3th-kiaaa-aaaaq-aab6q-cai-nif4qry.7776d299b4a804a14862b02bff7b74d1b956e431f5f832525d966d67ff3d7ce8";

const URI_SUPPLY =
  "https://icrc-api.internetcomputer.org/api/v1/ledgers/tyyy3-4aaaa-aaaaq-aab7a-cai/total-supply";

export const treasuryData = async () => {
  try {
    const uri = `${URI}/balance-history?start=${1671580800}&step=${86400}`;
    const res = await fetch(uri);
    const treasury = await res.json();
    const data = treasury.data?.map((t) => ({ label: t.day, value: t.balance / 10e7 }));
    return data;
  } catch (err) {
    console.log("treasury data error:", err);
    return [];
  }
};

export const supplyData = async () => {
  try {
    const url = `${URI_SUPPLY}/?start=${1702404000}&step=${86400}`;
    const res = await fetch(url);
    const totalSupply = await res.json();
    const data = totalSupply.data?.map((t) => ({
      label: parseTimestamp(t[0]),
      value: t[1] / 10e7,
    }));
    return data;
  } catch (err) {
    console.log("totalSupply data error:", err);
    return [];
  }
};
