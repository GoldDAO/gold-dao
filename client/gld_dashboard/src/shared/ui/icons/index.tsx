import {
  CheckIcon as Check,
  ChevronUpDownIcon as ChevronUpDown,
  ChevronDownIcon as Chevron,
  PauseCircleIcon as PauseCircle,
  ArrowPathIcon as Pending,
  ArrowPathIcon as Swap,
  CheckCircleIcon as SuccessCircle,
  XMarkIcon as Close,
  XCircleIcon as ErrorCircle,
  ArrowLeftStartOnRectangleIcon as Logout,
  ArrowTopRightOnSquareIcon as ExternalLink,
  ArrowsUpDownIcon as Transfer,
  ArrowDownIcon as Arrow,
  ClipboardDocumentIcon as Copy,
  UserIcon as User,
  FireIcon as Burn,
  // CursorArrowRaysIcon as Mint,
  InformationCircleIcon as InfoCircle,
  Bars3Icon as Menu,
  PlusIcon as Plus,
  MinusIcon as Minus,
  ChevronDoubleDownIcon as ChevronDouble,
  ClockIcon as Clock,
} from "@heroicons/react/24/outline";
import { ExclamationTriangleIcon as Warning } from "@heroicons/react/24/solid";
import {
  Wallet,
  Speedometer,
  BuyCrypto,
  Magicpen as Mint,
  Sun1 as Sun,
  Moon,
  Monitor,
} from "iconsax-react";

const RedeemIcon = () => {
  return (
    <svg
      width="25"
      height="16"
      viewBox="0 0 25 16"
      className="fill-current stroke-current"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M12.6996 6.97288C12.3443 7.15527 12.1316 7.53158 12.1586 7.93005L12.4971 12.931C12.5516 13.7364 13.4888 14.1486 14.1193 13.6445L23.066 6.49147C23.3962 6.22741 23.5247 5.78415 23.3869 5.38439L22.6209 3.16334C22.4224 2.58794 21.7603 2.32178 21.2188 2.59973L12.6996 6.97288ZM4.31696 6.02318C3.82452 5.93727 3.34429 6.2282 3.1924 6.70444L2.01763 10.3877C1.85212 10.9066 2.13236 11.4626 2.64796 11.6382L10.8652 14.4367C11.5411 14.6669 12.2335 14.1349 12.1853 13.4225L11.8252 8.10644C11.7941 7.64689 11.4531 7.26804 10.9994 7.18889L4.31696 6.02318ZM3.21843 5.466L15.6697 0.515386L22.4541 1.59577L24.0461 6.21169L12.5058 15.4384L1.25916 11.6083L3.21843 5.466ZM11.5802 6.85753C11.7945 6.89491 12.0151 6.86139 12.2087 6.76204L18.9491 3.30212C19.8091 2.86068 19.6046 1.57716 18.6499 1.42495L15.9654 0.996916C15.7877 0.968587 15.6057 0.988717 15.4385 1.05518L7.40414 4.24926C6.46687 4.62188 6.60813 5.99035 7.60176 6.16365L11.5802 6.85753Z" />
    </svg>
  );
};

const EarnIcon = () => {
  return (
    <svg
      width="25"
      height="24"
      viewBox="0 0 25 24"
      xmlns="http://www.w3.org/2000/svg"
      className="fill-current"
    >
      <path d="M22.3673 4.94704L17.6985 5.50954C17.5438 5.52829 17.4782 5.71813 17.5884 5.82829L18.9782 7.21813L13.6813 12.515L11.2954 10.1314C11.1477 9.98375 10.911 9.9861 10.7657 10.1314L3.22586 17.6736C3.19096 17.7088 3.17139 17.7564 3.17139 17.806C3.17139 17.8556 3.19096 17.9032 3.22586 17.9384L4.28054 18.9978C4.3532 19.0705 4.47273 19.0705 4.54539 18.9978L11.0305 12.515L13.4141 14.8986C13.5618 15.0439 13.7985 15.0439 13.9438 14.8986L20.3048 8.54235L21.6946 9.93219C21.7195 9.95698 21.7509 9.97427 21.7851 9.98211C21.8194 9.98996 21.8551 9.98803 21.8883 9.97655C21.9216 9.96508 21.9509 9.94451 21.973 9.9172C21.9951 9.88988 22.0091 9.85691 22.0134 9.82204L22.5759 5.15329C22.5923 5.03375 22.4891 4.93063 22.3673 4.94704Z" />
    </svg>
  );
};

const GovernIcon = () => {
  return (
    <svg
      width="25"
      height="24"
      viewBox="0 0 25 24"
      className="fill-current"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M12.75 3C7.77944 3 3.75 7.02944 3.75 12C3.75 16.9706 7.77944 21 12.75 21C17.7206 21 21.75 16.9706 21.75 12C21.75 7.02944 17.7206 3 12.75 3ZM1.75 12C1.75 5.92487 6.67487 1 12.75 1C18.8251 1 23.75 5.92487 23.75 12C23.75 18.0751 18.8251 23 12.75 23C6.67487 23 1.75 18.0751 1.75 12Z" />
      <path d="M17.6969 7.05269C17.9647 7.32052 18.0583 7.71669 17.9385 8.07602L15.8185 14.436C15.7189 14.7346 15.4846 14.9689 15.186 15.0685L8.82602 17.1885C8.46669 17.3083 8.07052 17.2147 7.80269 16.9469C7.53485 16.6791 7.44133 16.2829 7.56111 15.9236L9.68111 9.56356C9.78064 9.26496 10.015 9.03064 10.3136 8.93111L16.6736 6.81111C17.0329 6.69133 17.4291 6.78485 17.6969 7.05269ZM11.4204 10.6704L10.0909 14.6587L14.0792 13.3292L15.4087 9.34093L11.4204 10.6704Z" />
    </svg>
  );
};

const BuyOnBity = () => {
  return (
    <svg
      width="17"
      height="22"
      viewBox="0 0 17 22"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M12.9061 0H3.39257C1.83581 0 0.625 1.14925 0.625 2.54478V19.3731C0.625 20.8507 1.83581 22 3.39257 22H12.9061C14.9818 22 16.625 20.4403 16.625 18.5522V13.0522C16.625 12.1493 16.0196 11.4104 15.0682 11.1642L14.5493 11L15.0682 10.8358C16.0196 10.5896 16.625 9.85075 16.625 8.94776V3.44776C16.625 1.5597 14.9818 0 12.9061 0ZM13.5115 13.6269V18.0597C13.5115 18.7985 12.9061 19.3731 12.1277 19.3731H9.10068C8.3223 19.3731 7.71689 18.7985 7.71689 18.0597V13.6269C7.71689 12.8881 8.3223 12.3134 9.10068 12.3134H12.1277C12.9061 12.3134 13.5115 12.8881 13.5115 13.6269ZM13.5115 4.02239V8.37313C13.5115 9.11194 12.9061 9.68657 12.1277 9.68657H9.10068C8.3223 9.68657 7.71689 9.11194 7.71689 8.37313V4.02239C7.71689 3.28358 8.3223 2.70896 9.10068 2.70896H12.1277C12.9061 2.70896 13.5115 3.28358 13.5115 4.02239Z"
        fill="white"
      />
    </svg>
  );
};

const Icon = {
  User,
  BuyOnBity,
  Copy,
  Arrow,
  Redeem: RedeemIcon,
  Earn: EarnIcon,
  Govern: GovernIcon,
  Check,
  Chevron,
  Transfer,
  ExternalLink,
  Logout,
  SuccessCircle,
  ErrorCircle,
  Pending,
  PauseCircle,
  Close,
  ChevronUpDown,
  Burn,
  Mint,
  InfoCircle,
  Menu,
  Plus,
  Minus,
  ChevronDouble,
  Warning,
  Clock,
  Wallet,
  Speedometer,
  Swap,
  BuyCrypto,
  Sun,
  Moon,
  Monitor,
};

export default Icon;
