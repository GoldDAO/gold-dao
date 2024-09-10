import axios from "axios";
import { API_OGY_BASE_URL } from "@constants";

export const instance = axios.create({
    baseURL: API_OGY_BASE_URL,
  });

export * from "./useBalanceOGYUSD"