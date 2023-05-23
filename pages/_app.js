import { Inter } from 'next/font/google';
import './../src/css/global.css'
import styled from 'styled-components';
import { Box } from "@mui/system";
import {Button} from '@mui/base';

const inter = Inter({ subsets: ['latin'] });
 
export default function MyApp({ Component, pageProps }) {
  return (
    <main className={inter.className}>
      <Component {...pageProps} />
    </main>
  );
}

export const PageContent = styled(Box)`
  h1{
    font-weight: 300;
    padding: 26px 0;
    font-size: 6.4em;
  }
  h2{
    font-weight: 600;
    font-size: 2.4em;
    padding: 26px 0;
    color: #333;
  }
  p{
    font-weight: 300;
    line-height: 2em;
    padding: 26px 0;
  }
`

export const PrimaryButton = styled(Button)`
    height: fit-content;
    padding: 10px 25px;
    background-color: #D3B872;
    color: #fff;
    border-radius: 10px;
    font-size: 1em;
    border: 0;
    cursor: pointer;
    outline: none;
    box-shadow: none;
`