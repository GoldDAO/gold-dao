import NavbarTop from "@components/shared/navbars/NavbarTop";
import { useEffect, useState } from "react";
import Hero from "./Hero";
import UnlockPotentialGold from "./sections/UnlockPotentialGold";
import WhyChooseGLDT from "./sections/WhyChooseGLDT";
import WhatIsGLDT from "./sections/WhatIsGLDT";
import Transparency from "./sections/Transparency";
import Ecosystem from "./sections/Ecosystem";
import Partners from "./sections/Partners";
import FAQ from "./sections/FAQ";
import Footer from "./Footer";

const Home = () => {
  const [isDark, setIsDark] = useState(false);

  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    setIsDark(mq.matches);
    const handler = (e: MediaQueryListEvent) => setIsDark(e.matches);
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, []);
  return (
    <main>
      <header className="bg-gold/15 sticky top-0 z-10 min-h-[100vh] flex flex-col overflow-hidden">
        <video
          className="absolute inset-0 w-full h-full object-cover z-0"
          src={
            isDark
              ? "https://sos-ch-gva-2.exo.io/daolink-gold-dao-website-medias/landing_page/bg_hero_dark.mov"
              : "https://sos-ch-gva-2.exo.io/daolink-gold-dao-website-medias/landing_page/bg_hero_light.mov"
          }
          autoPlay
          loop
          muted
          playsInline
          aria-label="Background hero video"
          style={{ height: "100%", minHeight: "100%", maxHeight: "100%" }}
        />
        <div className="relative z-10 w-full">
          <NavbarTop />
        </div>
        <div className="relative z-10 w-full flex-1 flex flex-col justify-center items-center">
          <Hero />
        </div>
      </header>

      <div className="relative z-20 bg-background rounded-t-[50px]">
        <UnlockPotentialGold className="bg-surface-2 rounded-t-full py-16 md:py-24" />

        <section>
          <div className="container mx-auto bg-surface-1 md:rounded-xl">
            <WhyChooseGLDT className="md:rounded-[inherit]" />
            <WhatIsGLDT id="gldt" />
          </div>
        </section>

        <Transparency id="transparency" className="py-16 md:py-24" />

        <Ecosystem id="ecosystem" className="bg-surface py-16 md:py-24" />

        <Partners id="partners" className="bg-surface-invert py-16 md:py-24" />

        <FAQ id="faq" className="py-16 md:py-24" />

        <Footer />
      </div>
    </main>
  );
};

export default Home;
