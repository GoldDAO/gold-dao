import { Link } from "react-router-dom";
import { Button } from "@components/index";
import { Logo } from "@components/logos";
import { useTranslation, Trans } from "react-i18next";

const Dexs = ({ className }: { className?: string }) => {
  const { t } = useTranslation("dashboard/dexs");

  const dexs = [
    {
      name: "ICP Swap",
      logo: <Logo name="icpswap" className="h-4 w-4" />,
      url: "https://app.icpswap.com/swap?input=ryjl3-tyaaa-aaaaa-aaaba-cai&output=tyyy3-4aaaa-aaaaq-aab7a-cai",
    },
    {
      name: "Sonic",
      logo: <Logo name="sonic" className="h-4 w-4" />,
      url: "https://app.sonic.ooo/?tokenin=ryjl3-tyaaa-aaaaa-aaaba-cai&tokenout=tyyy3-4aaaa-aaaaq-aab7a-cai",
    },
    {
      name: "ICDEX",
      logo: <Logo name="icdex" className="h-4 w-4" />,
      url: "https://iclight.io/wallet",
    },
    {
      name: "KongSwap",
      logo: <Logo name="kongswap" className="h-4 w-4" />,
      url: "https://www.kongswap.io/?viewtab=swap&pool=ICP_GLDGov",
    },
    {
      name: "LBank",
      logo: <Logo name="lbank" className="h-4 w-4" />,
      url: "https://www.lbank.com/",
    },
  ];

  return (
    <div className={className}>
      <div className="border border-border rounded-xl bg-surface-secondary bg-[url('/src/assets/bg-cover.png')] bg-cover relative h-96">
        <img
          className="rounded-xl absolute h-96"
          src="./gold_bars.svg"
          alt="Gold bars"
        />

        <div className="relative z-10 h-full">
          <div className="grid grid-cols-1 xl:grid-cols-4 h-full">
            <div className="flex items-center justify-center h-full xl:col-start-2 xl:col-span-3">
              <div className="text-center">
                <h1 className="text-5xl">
                  <Trans
                    t={t}
                    i18nKey="trade_on"
                    components={{
                      1: <span className="text-accent font-bold" />,
                    }}
                  />
                </h1>
                <div className="mt-6">
                  <div className="flex flex-col xl:flex-row justify-center items-center gap-3">
                    {dexs.map((dex) => (
                      <div key={dex.name}>
                        <Link
                          to={dex.url}
                          target="_blank"
                          rel="noopener noreferrer"
                        >
                          <Button className="flex items-center gap-2 px-4">
                            {dex.logo}
                            <div>{dex.name}</div>
                          </Button>
                        </Link>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dexs;
