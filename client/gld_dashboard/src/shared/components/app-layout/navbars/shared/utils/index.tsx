import { ReactNode } from "react";
import Icon from "@shared/ui/icons";

const navItems: {
  title: string;
  url: string;
  icon: ReactNode;
  subtitle?: string;
}[] = [
  {
    title: "Buy",
    url: "/buy",
    icon: <Icon.Redeem />,
  },
  {
    title: "Earn",
    subtitle: "Coming Soon",
    url: "/earn",
    icon: <Icon.Earn />,
  },
  {
    title: "Govern",
    url: "/govern",
    icon: <Icon.Govern />,
  },
  { title: "Wallet", url: "/wallet", icon: <Icon.Wallet width={24} /> },
  {
    title: "Advanced",
    url: "/advanced/gldt",
    icon: <Icon.Speedometer width={24} />,
  },
];

export default navItems;
