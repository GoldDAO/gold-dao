'use client';

import '../../app/globals.css';
import 'react-toastify/dist/ReactToastify.css';

import { Inter } from 'next/font/google';

import Footer from '../shared/Footer';
import { FooterGoldDaoIcon } from '../../utils/svgs';

const inter = Inter({ subsets: ['latin'] });

export const metadata = {
  title: 'Gold DAO - Dashboard',
  description:
    'Access the Gold DAO Dashboard to manage your GLDGov tokens, view neuron ownership details, and claim your rewards. Enhance your DAO experience with comprehensive tools and real-time insights.',
  keywords: ['RWA', 'DAO', 'WEB3', 'GOLD', 'BLOCKCHAIN'],
};

export const viewport = {
  themeColor: '#c6c6c6',
};

export default function MaintenanceLayout() {
  return (
      <html lang="en">
        <head>
          <meta name="theme-color" content="#c6c6c6" />
          <meta name="robots" content="noindex" />
        </head>
          <body className={`${inter.className} flex h-screen bg-white text-black`}>
            <section className="w-full overflow-y-scroll flex flex-col justify-between ">
              <div className="flex py-16  justify-center text-center align-center items-center flex-col">
                <FooterGoldDaoIcon w={188} h={60} />
                <h2 class="font-bold">Site is under maintenance.</h2>
                <p class="mt-3">We&apos;re just upgrading the site. We&apos;ll be back very soon.</p>
              </div>
              <Footer />
            </section>
          </body>
      </html>
  );
}
