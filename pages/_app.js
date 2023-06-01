import { Inter } from 'next/font/google';
import './../src/css/global.css'
import styled from 'styled-components';
import { Box } from "@mui/system";
import { Button } from '@mui/base';
import { Provider, createStore } from 'jotai'


const myStore = createStore()
const inter = Inter({ subsets: ['latin'] });

export default function MyApp({ Component, pageProps }) {
  return (
    <main className={inter.className}>
      <Provider store={myStore}>
        <Component {...pageProps} />
      </Provider>
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

