import { atom } from "jotai";

import { Tab, TabList } from "@advanced/shared/utils";

const TabSelectedAtom = atom<Tab>(TabList[0]); // default to GLDT

export default TabSelectedAtom;
