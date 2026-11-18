import axios from "axios";
import { GLDT_API_BASE_URL } from "@constants";

const instance = axios.create({
  baseURL: GLDT_API_BASE_URL,
});

// Latest gold price in USD per gram. The endpoint answers with a bare number.
export const fetch_gold_price = async (): Promise<number> => {
  const { data } = await instance.get(`/tokens/gold/price/latest`);
  if (typeof data !== "number" || !Number.isFinite(data)) {
    throw new Error(`Unexpected gold price: ${JSON.stringify(data)}`);
  }
  return data;
};
