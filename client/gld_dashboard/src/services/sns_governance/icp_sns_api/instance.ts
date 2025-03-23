import axios from "axios";

const ICP_SNS_API_BASE_URL="https://sns-api.internetcomputer.org/api/v1"

const instance = axios.create({
  baseURL: ICP_SNS_API_BASE_URL,
  headers: {
    "Content-Type": "application/json",
  },
});

export default instance