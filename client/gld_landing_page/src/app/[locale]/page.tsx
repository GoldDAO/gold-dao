
import GoldDAOChart from "@/components/GoldDAOChart";
import Governance from "@/components/Governance";
import Header from "@/components/Header";
import Hero from "@/components/Hero";
import Partners from "@/components/Partners";
import TokensCards from "@/components/TokensCards";
import TokensInfos from "@/components/TokensInfos";
import WhyInfos from "@/components/WhyInfos";
export default async function Home() {
  return (
    <main className="flex flex-col items-center">
      <Header />
      <Hero />
      <TokensInfos />
      <WhyInfos />
      <GoldDAOChart />
      <TokensCards />
      <Partners />
      <Governance />
    </main>
  );
}
