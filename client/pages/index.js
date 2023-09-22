import Layout from '@/components/layout/Layout';
import Head from 'next/head';

function Home({}) {
    const meta = {
        title: 'GLDT Swap App',
        description: 'GLDT Swap App Description',
    };
    return (
        <>
            <Head>
                <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" />
            </Head>
            <Layout></Layout>
        </>
    );
}

export default Home;
