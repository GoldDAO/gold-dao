import axios from "axios";
import { OGY_API_BASE_URL } from "@constants";

export const instance = axios.create({
  baseURL: OGY_API_BASE_URL,
});
