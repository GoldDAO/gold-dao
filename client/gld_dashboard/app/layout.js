import './globals.css';
import 'react-toastify/dist/ReactToastify.css';

import { Inter } from 'next/font/google';
import { ToastContainer } from 'react-toastify';
import getConfig from 'next/config';

import Footer from '../components/shared/Footer';
import Maintenance from '../components/Maintenance/index';
import Header from '../components/shared/Header/Header';
import Navbar from '../components/shared/Navbar';
import Providers from './providers';

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

export default function RootLayout({ children }) {
  const isMaintenanceMode = process.env.MAINTENANCE_MODE;
  console.log(process.env);
  console.log(getConfig());
  console.log(isMaintenanceMode);

  if (isMaintenanceMode) {
    return <Maintenance />;
  }

  return (
    <html lang="en">
      <head>
        <meta name="theme-color" content="#c6c6c6" />
      </head>
      <Providers>
        <body className={`${inter.className} flex h-screen bg-white text-black`}>
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
        </body>
      </Providers>

      <script defer data-domain="dashboard.gold-dao.org" src="https://analytics.gold-dao.org/js/script.js"></script>
    </html>
  );
}
