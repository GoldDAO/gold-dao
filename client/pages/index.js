import Head from 'next/head';
import Layout from '@/components/UI/layout/Layout';
import { getMarkdownPage, getMarketCap, getPartners, getSwapCTO } from '@/utils/getMarkdown';
import { markdownToHtml } from '@/utils/markdownToHtml';
import Marketcap from '@/components/UI/sections/Marketcap';
import { PageContent } from './_app';
import Partners from '@/components/UI/sections/partners';
import Chart from '@/components/UI/sections/Chart';
import dynamic from 'next/dynamic';

function Home({ content, meta, partners, cto, marketcap }) {
  const Swap = dynamic(() => import('@/components/UI/sequence/SwapContainer'), {
    ssr: false,
  });

  const MyNfts = dynamic(() => import('@/components/UI/table/NftsTable'), {
    ssr: false,
  });


  return (
    <>
      <Head>
        <title>{meta.title}</title>
        <meta property={`og:title`} content={meta.title} key="title" />
        <meta property={`og:description`} content={meta.description} key="title" />
      </Head>
      <Layout>
        <Marketcap data={marketcap.data} />
        <PageContent dangerouslySetInnerHTML={{ __html: content }} />
        <Swap data={cto.data} />
        <Chart />
        <MyNfts />
        <Partners partners={partners} />
      </Layout>
    </>
  );
}

export default Home;

export async function getStaticProps() {
  const content = getMarkdownPage('home');
  const html = await markdownToHtml(content.content);
  const partners = await getPartners();
  const CTO = await getSwapCTO();
  const marketcap = await getMarketCap();
  return {
    props: {
      content: html,
      meta: content.data,
      partners: partners,
      cto: CTO,
      marketcap: marketcap,
    },
  };
}
