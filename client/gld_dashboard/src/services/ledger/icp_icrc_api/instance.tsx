import axios from "axios";

const ICP_ICRC_API_BASE_URL = "https://icrc-api.internetcomputer.org/api/v1";

export const instance = axios.create({
  baseURL: ICP_ICRC_API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});
