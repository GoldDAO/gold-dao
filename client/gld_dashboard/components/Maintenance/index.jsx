'use client';

import '../../app/globals.css';
import 'react-toastify/dist/ReactToastify.css';

import { Inter } from 'next/font/google';

import Footer from '../shared/Footer';

const inter = Inter({ subsets: ['latin'] });

export const metadata = {
  title: 'Gold DAO - Dashboard',
  description:
    'Access the Gold DAO Dashboard to manage your GOLDAO tokens, view neuron ownership details, and claim your rewards. Enhance your DAO experience with comprehensive tools and real-time insights.',
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
                <img src="svg/logo-full.svg" alt="" className="w-[188px]" />
                <h2 className="font-bold">Site is under maintenance.</h2>
                <p className="mt-3">We&apos;re just upgrading the site. We&apos;ll be back very soon.</p>
              </div>
              <Footer />
            </section>
          </body>
      </html>
  );
}
