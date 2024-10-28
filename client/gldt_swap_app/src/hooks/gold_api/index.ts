import axios from "axios";
import { DateTime } from "luxon";
import { GOLD_API_BASE_URL } from "@constants";

const instance = axios.create({
    baseURL: GOLD_API_BASE_URL,
    headers: {
      // "x-access-token": GOLD_API_API_KEY,
      "Content-Type": "application/json"
    }
  });

export const fetchGoldPrice1G = async () => {
  const dateOfTheDay = DateTime.now().toFormat('yyyy-MM-dd')
    const { data } = await instance.get(`/rates/XAUUSD?start_date=${dateOfTheDay}&end_date=${dateOfTheDay}`);
    const { rates }: {
      rates: {
        [date: string]: {
          USDXAU: number;
          XAU: number;
        };
      };
    } = data;
    return Object.values(rates)[0]["USDXAU"];
  };

export * from "./useBalanceGLDTUSD"