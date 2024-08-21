'use client';

import './globals.css';
import 'react-toastify/dist/ReactToastify.css';
import { useEffect, useState } from 'react';

import { Inter } from 'next/font/google';
import { ToastContainer } from 'react-toastify';
import Head from 'next/head';
import Footer from '../components/shared/Footer';
import Maintenance from '../components/Maintenance/index';
import Header from '../components/shared/Header/Header';
import Navbar from '../components/shared/Navbar';
import Providers from './providers';
import useManagement from '../hooks/useManagement';

const inter = Inter({ subsets: ['latin'] });

export const viewport = {
  themeColor: '#c6c6c6',
};

export default function RootLayout({ children }) {
  const [isMaintenanceMode, setMaintenanceMode] = useState(false);
  const { getMaintenanceMode } = useManagement();

  useEffect(() => {
    // icp balance
    (async () => {
      const mode = await getMaintenanceMode();
      setMaintenanceMode(mode);
    })();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isMaintenanceMode]);

  if (isMaintenanceMode) {
    return <Maintenance />;
  }

  return (
    <html lang="en">
      <Head>
        <title>Gold DAO - Dashboard</title>
        <meta name="description" content="Access the Gold DAO Dashboard to manage your GLDGov tokens, view neuron ownership details, and claim your rewards. Enhance your DAO experience with comprehensive tools and real-time insights." />
        <meta name="keywords" content="RWA, DAO, WEB3, GOLD, BLOCKCHAIN" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="theme-color" content="#c6c6c6" />
        <script async="" data-domain="dashboard.gold-dao.org" src="https://analytics.gold-dao.org/js/script.js"></script>
      </Head>
      <body>
        <Providers>
          <div className={`${inter.className} flex h-screen bg-white text-black`}>
            <Navbar />
            <section className="w-full overflow-y-scroll flex flex-col justify-between ">
              <div className="px-5 sm:px-0">
                <Header />
                {children}
              </div>
              <Footer />
            </section>
            <ToastContainer
              position="top-right"
              autoClose={5000}
              hideProgressBar={false}
              newestOnTop={false}
              closeOnClick
              rtl={false}
              pauseOnFocusLoss
              draggable
              pauseOnHover
              theme="light"
              transition="Bounce"
            />
          </div>
        </Providers>
      </body>
    </html>
  );
}
