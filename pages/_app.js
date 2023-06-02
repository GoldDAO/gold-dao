import { Inter } from 'next/font/google';
import './../src/css/global.css'
import styled from 'styled-components';
import { Box } from "@mui/system";
import "@connect2ic/core/style.css"
import { Provider, createStore } from 'jotai'
import dynamic from 'next/dynamic';

const myStore = createStore()
const inter = Inter({ subsets: ['latin'] });



export default function MyApp({ Component, pageProps }) {

  const C2ICProvider = dynamic(() => import("./../lib/components/C2ICProvider"), {
    ssr: false,
  });

  function SafeHydrate({ children }) {
    return (
      <div suppressHydrationWarning>
        {typeof window === 'undefined' ? null : children}
      </div>
    )
  }

  return (
    <div className={inter.className}>
      <Provider store={myStore}>
        <SafeHydrate>
          <C2ICProvider>
            <Component {...pageProps} />
          </C2ICProvider >
        </SafeHydrate>
      </Provider>
    </div >
  )
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

