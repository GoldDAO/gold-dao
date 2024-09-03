
import Header from "@/components/Header";
import Hero from "@/components/Hero";
import TokensInfos from "@/components/TokensInfos";
import WhyInfos from "@/components/WhyInfos";
export default async function Home() {
  return (
    <main className="flex flex-col items-center px-2 md:px-10 ">
      <Header />
      <Hero />
      <TokensInfos />
      <WhyInfos />
    </main>
  );
}
