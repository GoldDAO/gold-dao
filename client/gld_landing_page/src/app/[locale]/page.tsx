import Header from "@/components/Header";
import Hero from "@/components/Hero";
export default async function Home() {
  return (
    <main className="flex flex-col items-center px-2 md:px-10 ">
      <Header />
      <Hero />
    </main>
  );
}
