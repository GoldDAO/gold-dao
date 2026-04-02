import { ReactNode } from "react";
import Icon from "@shared/ui/icons";

const navItems: {
  title: string;
  url: string;
  icon: ReactNode;
}[] = [
  {
    title: "Govern",
    url: "/govern",
    icon: <Icon.Govern />,
  },
  {
    title: "Earn",
    url: "/earn",
    icon: <Icon.Earn />,
  },
  { title: "Wallet", url: "/wallet", icon: <Icon.Wallet width={24} /> },
  // {
  //   title: "Advanced",
  //   url: "/advanced/gldt",
  //   icon: <Icon.Speedometer width={24} />,
  // },
];

export default navItems;
