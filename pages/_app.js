import { Inter } from 'next/font/google';
import './../src/css/global.css'
import styled from 'styled-components';
import { Box } from "@mui/system";
import { Button } from '@mui/base';
import { Provider } from 'jotai'
// import { createClient } from "@connect2ic/core"
// import { defaultProviders } from "@connect2ic/core/providers"
import "@connect2ic/core/style.css"

const inter = Inter({ subsets: ['latin'] });

// const client = createClient({
//   providers: defaultProviders,
//   globalProviderConfig: {
//     /*
//      * Disables dev mode in production
//      * Should be enabled when using local canisters
//      */
//     dev: true
//   },
// })

export default function MyApp({ Component, pageProps }) {
  return (
    <main className={inter.className}>
      <Provider
      // client={client}
      >
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