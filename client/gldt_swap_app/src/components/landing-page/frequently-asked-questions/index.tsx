import { ReactNode } from "react";
import { Link } from "react-router-dom";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import { Accordion, AccordionItem as Item } from "@szhsin/react-accordion";

const Question = ({ children }: { children: ReactNode }) => {
  return <div className="font-semibold text-left">{children}</div>;
};

const AccordionItem = ({
  header,
  ...rest
}: {
  header: ReactNode;
  children: ReactNode;
}) => (
  <Item
    {...rest}
    header={({ state: { isEnter } }) => (
      <div className="flex w-full items-center justify-between border-t border-gold/60 py-2">
        {header}
        <ChevronDownIcon
          className={`size-5 fill-content ml-auto transition-transform duration-300 ease-out ${
            isEnter && "rotate-180"
          }`}
        />
      </div>
    )}
    buttonProps={{
      className: () => `flex w-full py-4 text-left`,
    }}
    contentProps={{
      className: "transition-height duration-300 ease-out",
    }}
    panelProps={{ className: "" }}
  />
);

export const FrequentlyAskedQuestions = ({
  className = "",
  limit = 0,
}: {
  className?: string;
  limit?: number;
}) => {
  const faqs: { q: ReactNode; a: ReactNode }[] = [
    {
      q: <Question>What is a GLDT?</Question>,
      a: (
        <div>
          GLDT stands for “Gold Token”, being a fungible token backed by GLD
          NFTs.
          <br />
          GLD NFTs are NFTs that represent irrefutable ownership of physical
          gold bars.
          <br />1 GLDT represents exactly 0.01g of gold, and GLD NFTs can be
          swapped at a ratio of 1g for 100 GLDTs.",
        </div>
      ),
    },
    {
      q: <Question>How are GLDT minted?</Question>,
      a: (
        <div>
          GLDT are minted when someone swaps a GLD NFT for GLDT.
          <br />
          The swapping ratio is 1g of GLD NFT for 100 GLDT.
        </div>
      ),
    },
    {
      q: <Question>Who can participate in the swap?</Question>,
      a: (
        <div>
          Everybody who owns GLD NFT. See here for details of purchasing GLD
          NFTs.
        </div>
      ),
    },
    {
      q: <Question>How does the swap work?</Question>,
      a: (
        <div>
          You must own at least one GLD NFT and connect to the swap app.
          <br />
          Then you will be able to select the GLD NFT which you want to swap to
          GLDT.
          <br />
          After validating the transaction, the GLD NFT will be swapped against
          GLDT that you will see in your balance/wallet.
        </div>
      ),
    },
    {
      q: <Question>How is the GLDT price determined?</Question>,
      a: (
        <div>
          1g of gold = 100 GLDT. Or 1 GLDT = 0.01g of gold.
          <br />
          E.g.: You have 5g of GLD NFTs. You can swap these at a ratio of 1g to
          100GLDT. So when swapping 5g of GLD NFT, you will receive 500 GLDT.
        </div>
      ),
    },
    {
      q: <Question>How and when will I know if my swap is final?</Question>,
      a: (
        <div>
          Once the transaction is completed, you can find GLDT in your wallet
          balance.
        </div>
      ),
    },
    {
      q: <Question>What are the commissions for swapping GLDT?</Question>,
      a: (
        <div>
          Commissions are paid to Originator- BAS (0.4%), Authenticator- BAS
          (0.1%), ORIGYN (0.5%) and are a total of 1%.
          <br />
          These fees are integrated inside the GLD NFT. The swap fees are
          refunded for the first 100 million GLDT.
          <br />
          In addition to the swap fees, there is an inherent transaction fee of
          0.0001 GLDT to avoid spamming of the network.
        </div>
      ),
    },
    {
      q: <Question>What are the commissions on trading?</Question>,
      a: (
        <div>
          There are no fees on trading. However, some exchanges can apply their
          own exchange fees.
        </div>
      ),
    },
    {
      q: (
        <Question>
          How and when can I swap back my GLDT?
          <br />
          Can I sell my GLDT on another platform?
        </Question>
      ),
      a: (
        <div>
          You can swap your GLDT back to GLD NFT on the platform soon. This is
          currently under development.
          <br />
          You may also visit any exchange that supports GLDT and trade it for
          other cryptocurrencies.
        </div>
      ),
    },
    {
      q: <Question>Where can I find more information about GLD NFTs?</Question>,
      a: (
        <div>
          Go to{" "}
          <Link
            to="https://gold.bity.com"
            target="_blank"
            rel="noopener noreferrer"
            className="text-gold/80"
          >
            bity.gold.com
          </Link>{" "}
          to learn more about GLD NFTs, which back GLDTs.
        </div>
      ),
    },
  ];
  const _faqs = limit ? faqs.slice(0, limit) : faqs;
  return (
    <div className={className}>
      <Accordion transition transitionTimeout={250}>
        {_faqs.map(({ q, a }, index) => (
          <AccordionItem key={index} header={q}>
            {a}
          </AccordionItem>
        ))}
      </Accordion>
    </div>
  );
};
