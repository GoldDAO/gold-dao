import { ReactNode } from "react";
import { Wallet, Speedometer } from "iconsax-react";
import Earn from "@components/icons/Earn";
import Govern from "@components/icons/Govern";
import Redeem from "@components/icons/Redeem";

const navItems: {
  title: string;
  url: string;
  icon: ReactNode;
  subtitle?: string;
}[] = [
  {
    title: "Buy",
    url: "/buy",
    icon: <Redeem />,
  },
  {
    title: "Earn",
    subtitle: "Coming Soon",
    url: "/earn",
    icon: <Earn />,
  },
  {
    title: "Govern",
    url: "/govern",
    icon: <Govern />,
  },
  { title: "Wallet", url: "/wallet", icon: <Wallet /> },
  { title: "Advanced", url: "/advanced/gldt", icon: <Speedometer /> },
];

export default navItems;
