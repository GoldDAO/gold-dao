import axios from "axios";
import { GOLD_API_BASE_URL } from "@constants";

const instance = axios.create({
    baseURL: GOLD_API_BASE_URL,
    headers: {
      "Content-Type": "application/json"
    }
  });

export const fetchGoldPrice1G = async () => {
    const { data } = await instance.get(`/price/XAUUSD`);
    const { rates }: {
      rates: {
        USDXAU: number;
        XAU: number;
      };
    } = data;
    return rates.USDXAU / 31.103;
  };

export * from "./useBalanceGLDTUSD"
export * from "./useGLDTMarketcapUSD"