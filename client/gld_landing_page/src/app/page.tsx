import { Metadata } from "next";
import ClientSidePage from "../components/ClientSidePage";

export async function generateStaticParams() {
    const locales = ['en'];
    return locales.map((locale) => ({ locale }));
  }

  export const metadata: Metadata = {
  title: "The GOLD DAO Website",
  description:
    "Gold DAO is a decentralized community dedicated to revolutionizing the way gold is owned and utilised. Join the DAO in harnessing the power of blockchain technology to create a more transparent and accessible gold market.",
};
  
  export default function Page({ params }: { params: { locale: string } }) {
    return <ClientSidePage params={params} />;
  }