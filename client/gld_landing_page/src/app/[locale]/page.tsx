import Header from "@/components/Header";
export default async function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between bg-secondary px-2 md:px-10 ">
      <Header />
    </main>
  );
}
