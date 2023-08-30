import { Inter } from 'next/font/google';
import '@/css/global.css';
import styled from 'styled-components';
import { Box } from '@mui/system';
import '@connect2ic/core/style.css';
import dynamic from 'next/dynamic';
import { SafeHydrate } from '@/utils/SafeHydrate';
import Ruban from '@/components/UI/layout/Ruban';

const inter = Inter({ subsets: ['latin'] });

const Providers = dynamic(() => import('@/components/c2ic/Providers'), {
  ssr: false,
});

export default function MyApp({ Component, pageProps }) {
  return (
    <div className={inter.className}>
      {process.env.DFX_NETWORK !== 'production' &&
        <Ruban />
      }
      <Providers>
        <SafeHydrate>
          <Component {...pageProps} />
        </SafeHydrate>
      </Providers>
    </div>
  );
}

export const PageContent = styled(Box)`
  position: relative;
  h1 {
    mix-blend-mode: difference;
    font-weight: 300;
    padding: 26px 0;
    color: #333;
    max-width: 1200px;
    line-height: 1.05em;
    font-size: 5.4em;
    @media (max-width: 1140px) {
      line-height: 1em;
      font-size: 4.4em;
      padding-top: 20px;
      padding-bottom: 14px;
    }
    @media (max-width: 840px) {
      line-height: 1em;
      font-size: 3em;
    }
    @media (max-width: 480px) {
      line-height: 1em;
      font-size: 2.3em;
    }
  }
  h2 {
    font-weight: 300;
    font-size: 2em;
    padding: 26px 0;
    color: #333;
    @media (max-width: 1140px) {
      font-size: 1.8em;
    }
    @media (max-width: 840px) {
      font-size: 1.4em;
      padding: 15px 0;
    }
    @media (max-width: 480px) {
      font-size: 1.2em;
      padding: 10px 0;
    }
  }
  p {
    color: #333;
    font-weight: 300;
    line-height: 1.4em;
    padding: 26px 0;
    max-width: 1200px;
    @media (max-width: 1140px) {
    }
    @media (max-width: 840px) {
      font-size: 1em;
      padding: 15px 0;
    }
    @media (max-width: 480px) {
      font-size: 0.8em;
      padding: 10px 0;
    }
  }

  // &:before{
  //   width: 600px;
  //   height: 600px;
  //   content: "";
  //   border-radius: 500px;
  //   position: absolute;
  //   left: -30px;
  //   background-color: #d3b872;
  //   z-index: 200;
  //   top: -50px;
  //   opacity: .2;
  //   mix-blend-mode: difference;
  // }
`;
