import NavbarHome from "@components/shared/navbars/Home";

export const LandingPage = () => {
  return (
    <>
      <div className="bg-surface-2 bg-cover-img bg-cover bg-fixed">
        <div className="container mx-auto px-4">
          <NavbarHome />
        </div>
        <section className="p-16">
          <div className="flex justify-center items-center">
            <div className="order-last sm:order-first">GLDT ANIMATION</div>
            <div>
              <div>GLDT</div>
              <div>The future of owning physical gold</div>
              <div>BTN Start swapping</div>
              <div>
                <div>
                  <div>Total gold locked</div>
                  <div>(value)</div>
                </div>
                <div>
                  <div>GLDT marketcap in USD</div>
                  <div>(value)</div>
                </div>
              </div>
            </div>
          </div>
        </section>
      </div>
      <div className="container mx-auto">
        <section className="border-y border-gold/60">
          <div className="grid grid-cols-1 xl:grid-cols-2">
            <div className="flex justify-center px-4 xl:px-24 pt-16 pb-0 xl:py-24 text-center xl:text-left xl:border-r border-gold/60">
              <div className="text-xl">
                A token backed 100% in perpetuity by{" "}
                <span className="font-semibold">physical gold</span>
              </div>
            </div>
            <div className="flex justify-center px-4 xl:px-24 pt-8 pb-16 xl:py-24 text-center xl:text-left">
              <div className="">
                GLDT is a fractionable token backed by physical goldv. <br />
                Users can buy gold certificates (referred to as GLD NFTs) via
                the Yumi marketplace. These certificates can then be exchanged
                for GLDT tokens using the swap app. <br />
                For example, a GLD NFT representing 1 gram of gold can be
                swapped for 100 GLDT tokens.
                <br />
                Essentially, each GLDT represents a fractional share of actual
                physical gold, making it far more liquid. <br />
                This enables the trading of gold outside the traditional banking
                system.
              </div>
            </div>
          </div>
        </section>
        <section className="p-16">
          <div>Powered by</div>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6">
            <div>Logo</div>
            <div>Logo</div>
            <div>Logo</div>
            <div>Logo</div>
            <div>Logo</div>
            <div>Logo</div>
          </div>
        </section>
        <section className="p-16">
          <div>Our technology</div>
          <div>
            GLDTs and their underlying smart contracts run entirely on the ICP
            blockchain. In the future, GLDTs will become cross-platform and
            multi-chain. This heralds a new era in which physical gold can be
            transferred using blockchain technology. To learn more about how
            GLDT and swapping GLD NFTs works, please refer to the FAQ or review
            the whitepaper.
          </div>
        </section>
        <section className="border-y border-gold/60">
          <div className="grid grid-cols-1 xl:grid-cols-2">
            <div className="flex flex-col justify-center px-4 xl:px-24 pt-16 pb-0 xl:py-24 text-center xl:text-left xl:border-r border-gold/60">
              <div className="text-2xl">
                Determining the{" "}
                <span className="font-semibold">price of GLDT</span>
              </div>
              <div className="mt-8">
                The price of GLDT is directly correlated with the spot price of
                physical gold.
                <br />
                The market determines the price of gold, which is then used to
                calculate the value of GLDT.
                <br />
                It's important to note that every 100 GLDT equals 1 gram of
                gold.
                <br />
                This system operates 24/7, accessible all around the world with
                lowest fees.
              </div>
            </div>
            <div className="flex justify-center px-4 xl:px-24 pt-8 pb-16 xl:py-24 text-center xl:text-left bg-surface-2 bg-cover-img bg-cover">
              <img className="flex-none" src={`/swap_landing_page.svg`} />
            </div>
          </div>
        </section>

        <section className="p-16">
          <div>
            <div>Get started</div>
            <div>with GLD NFTs</div>
            <div>
              Explore the future of ownership of physical gold and acquire your
              GLD NFTs today on YUMI NFT marketplace.
            </div>
            <div>BTN Buy GLD NFTs</div>
          </div>
          <div>
            <div>
              <div>Frequently Asked Question</div>
              <div>View more FAQs</div>
            </div>
            <div>FAQ</div>
          </div>
        </section>
      </div>
    </>
  );
};
