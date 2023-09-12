import Head from 'next/head';
import Layout from '@/components/UI/layout/Layout';
import { getMarkdownPage, getMarketCap, getPartners, getSwapCTO } from '@/utils/getMarkdown';
import { markdownToHtml } from '@/utils/markdownToHtml';
import { PageContent } from './_app';
import Partners from '@/components/UI/sections/partners';
import Chart from '@/components/UI/sections/Chart';
import dynamic from 'next/dynamic';
import { useCanister } from '@connect2ic/react';
import TextSection from '@/components/UI/sections/TextSection;';
import Yumi from '@/components/UI/sections/Yumi';
import { CustomCircularProgress } from '@/components/UI/styled/common';
import AppStatus from '@/components/UI/feedback/AppStatus';

function Home({ content, meta, partners, cto, marketcap }) {
    const Banner = dynamic(() => import('@/components/UI/sections/Banner'), {
        ssr: false,
    });

    const textTitle = 'Gold. Blockchain. Secure. Stable. Simple.';
    const textContent =
        'GLDT is a stablecoin that is 100% backed by physical gold, making it a secure and reliable investment option in the world of cryptocurrency. GLDT is only minted when a GLD NFT is swapped, ensuring that it is fully backed by physical gold held in secure vaults in Switzerland.';

    return (
        <>
            <Head>
                <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" />
            </Head>
            <Layout>
                <Banner />
                <TextSection title={textTitle} content={textContent} />
                <Yumi />
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
