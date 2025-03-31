import axios from "axios";
import { BITY_GOLD_API_BASE_URL } from "@constants";

const instance = axios.create({
  baseURL: BITY_GOLD_API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

export const fetch_gold_price = async () => {
  const { data } = await instance.get(`/price/XAUUSD`);
  const {
    rates,
  }: {
    rates: {
      USDXAU: number;
      XAU: number;
    };
  } = data;
  return rates.USDXAU / 31.103;
};
