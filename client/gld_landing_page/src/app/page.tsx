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
    openGraph: {
      type: "website",
      url: "https://www.gold-dao.org/",
      title: "The GOLD DAO Website",
      description: "Gold DAO is a decentralized community dedicated to revolutionizing the way gold is owned and utilised. Join the DAO in harnessing the power of blockchain technology to create a more transparent and accessible gold market.",
      images: [
        {
          url: "https://www.gold-dao.org/static/illustrations/gold_dao.png",
          width: 800,
          height: 600,
          alt: "The GOLD DAO Website",
        },
      ],
    },
  
    twitter: {
      card: "summary_large_image",
      title: "The GOLD DAO Website",
      description: "Gold DAO is a decentralized community dedicated to revolutionizing the way gold is owned and utilised. Join the DAO in harnessing the power of blockchain technology to create a more transparent and accessible gold market.",
      images: ["https://www.gold-dao.org/static/illustrations/gold_dao.png"],
    },
};
  
  export default function Page({ params }: { params: { locale: string } }) {
    return <ClientSidePage params={params} />;
  }