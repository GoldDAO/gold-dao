/* eslint-disable react/jsx-no-undef */
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import initTranslations from "../i18n";
import TranslationsProvider from "@/providers/TranslationsProvider";
import i18nConfig from "../../../i18nConfig";
import "./globals.css";
import ReactQueryProvider from "@/providers/ReactQueryProvider";

const inter = Inter({ subsets: ["latin"] });

const i18nNamespaces = [
  "header",
  "hero",
  "tokens",
  "why",
  "goldDaoChart",
  "cards",
  "partners",
  "governance",
  "whitepaper",
  "footer",
];

export const metadata: Metadata = {
  title: "The GOLD DAO Website",
  description:
    "Gold DAO is a decentralized community dedicated to revolutionizing the way gold is owned and utilised. Join the DAO in harnessing the power of blockchain technology to create a more transparent and accessible gold market.",
};
export function generateStaticParams() {
  return i18nConfig.locales.map((locale) => ({ locale }));
}
export default async function RootLayout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
  params: { locale: string }; 
}>) {
  const { resources } = await initTranslations(params.locale, i18nNamespaces);

  return (
    <TranslationsProvider
      resources={resources}
      locale={params.locale}
      namespaces={i18nNamespaces}>
      <html lang={params.locale}>
        <body className={`${inter.className} bg-[#FAF9F8]`}>
          <ReactQueryProvider>{children}</ReactQueryProvider>
        </body>
      </html>
    </TranslationsProvider>
  );
}
