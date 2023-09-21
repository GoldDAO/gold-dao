import Head from 'next/head';
import Layout from '@/components/UI/layout/Layout';
import { getMarkdownPage, getPartners } from '@/utils/getMarkdown';
import { markdownToHtml } from '@/utils/markdownToHtml';
import Partners from '@/components/UI/sections/partners';
import dynamic from 'next/dynamic';
import TextSection from '@/components/UI/sections/TextSection;';
import Yumi from '@/components/UI/sections/Yumi';

function Home({ meta, partners }) {
    const Banner = dynamic(() => import('@/components/UI/sections/Banner'), {
        ssr: false,
    });

    const Marketcap = dynamic(() => import('@/components/UI/sections/Marketcap'), {
        ssr: false,
    });

    const textTitle = 'Gold. Blockchain. Secure. Stable. Simple.';
    const textContent =
        'GLDT is a token that is 100% backed by physical gold, making it a secure and reliable asset in the world of cryptocurrency. GLDT is only minted when a GLD NFT is swapped, ensuring that it is fully backed by physical gold held in secure vaults in Switzerland.';

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
                <Marketcap />
                <Partners partners={partners} />
                {/* <Chart /> */}
            </Layout>
        </>
    );
}

export default Home;

export async function getStaticProps() {
    const content = getMarkdownPage('home');
    const html = await markdownToHtml(content.content);
    const partners = await getPartners();
    return {
        props: {
            content: html,
            meta: content.data,
            partners: partners,
        },
    };
}
