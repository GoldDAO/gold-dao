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
          NFTs. GLD NFTs are NFTs that represent irrefutable ownership of
          physical gold bars. 1 GLDT represents exactly 0.01g of gold, and GLD
          NFTs can be swapped at a ratio of 1g for 100 GLDTs."
        </div>
      ),
    },
    {
      q: <Question>How are GLDT minted?</Question>,
      a: (
        <div>
          GLDT are minted when someone swaps a GLD NFT for GLDT. The swapping
          ratio is 1g of GLD NFT for 100 GLDT.
        </div>
      ),
    },
    {
      q: <Question>Who can participate in the swap?</Question>,
      a: (
        <div>
          Everybody who owns GLD NFT. See{" "}
          <Link
            to="https://gold.bity.com"
            target="_blank"
            rel="noopener noreferrer"
            className="text-gold/80"
          >
            gold.bity.com
          </Link>{" "}
          for details of purchasing GLD NFTs.
        </div>
      ),
    },
    {
      q: <Question>How does the swap work?</Question>,
      a: (
        <div>
          You must own at least one GLD NFT and connect to the swap app. Then
          you will be able to select the number of GLD NFT which you want to
          swap to GLDT. After validating the transaction, the GLD NFT will be
          locked in the swap canister and GLDT are effectively minted on the
          spot to your account. There will always be a ratio of 100GLDT to 1g
          GLD NFT owned by the swap canister.
        </div>
      ),
    },
    {
      q: <Question>How is the GLDT price determined?</Question>,
      a: (
        <div>
          1g of gold = 100 GLDT. Or 1 GLDT = 0.01g of gold. E.g.: You have 5g of
          GLD NFTs. You can swap these at a ratio of 1g to 100GLDT. So when
          swapping 5g of GLD NFT, you will receive 500 GLDT.
        </div>
      ),
    },
    {
      q: <Question>How and when will I know if my swap is final?</Question>,
      a: (
        <div>
          You can follow along your transaction in your Account section. Once
          the transaction is completed, you will find your newly minted GLDT in
          your wallet balance.
        </div>
      ),
    },
    {
      q: <Question>What are the fees for swapping GLDT?</Question>,
      a: (
        <div>
          There are no fees for swapping GLD NFT to GLDT. For reverse swapping
          from GLDT to GLD NFT, a fee of 1 GLDT is charged for every GLD NFT
          that is unlocked from the swap canister. For example, if 500 GLDT are
          swapped to 5x 1g GLD NFT, 5x 1 GLDT fee are applied. If 1000 GLDT are
          swapped to 1x 10g GLD NFT, 1x 1 GLDT will be applied.
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
            gold.bity.com
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
