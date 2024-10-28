import axios from "axios";
import { GOLD_API_BASE_URL, GOLD_API_API_KEY } from "@constants";

const instance = axios.create({
    baseURL: GOLD_API_BASE_URL,
    headers: {
      "x-access-token": GOLD_API_API_KEY,
      "Content-Type": "application/json"
    }
  });

export const fetchGoldPrice1G = async () => {
    const { data } = await instance.get(`/XAU/USD`);
    const { price } = data;
    return price;
  };

export * from "./useBalanceGLDTUSD"