/* eslint-disable react/jsx-no-undef */
import type { Metadata } from "next";
import { Inter } from "next/font/google";
import initTranslations from "../i18n";
import TranslationsProvider from "@/providers/TranslationsProvider";
import i18nConfig from '../../../i18nConfig';
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
    "A fungible cross-chain token that can act as a medium of exchange. GLDTs fractionalize gold ownership and serve as stable collateral for DeFi.",
};
export function generateStaticParams() {
  return i18nConfig.locales.map(locale => ({ locale}))
} 
export default async function RootLayout({
  children,
  locale,
}: Readonly<{
  children: React.ReactNode;
  locale: string;
}>) {
  const { resources } = await initTranslations(locale, i18nNamespaces);

  return (
    <TranslationsProvider
      resources={resources}
      locale={locale}
      namespaces={i18nNamespaces}>
      <html lang={locale}>
        <body className={`${inter.className} bg-[#FAF9F8]`}>
          <ReactQueryProvider>{children}</ReactQueryProvider>
        </body>
      </html>
    </TranslationsProvider>
  );
}
