import Layout from '@/components/layout/Layout';
import { cardPadding } from '@/theme/theme';
import { Card, Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import dynamic from 'next/dynamic';
import Head from 'next/head';

function Home({}) {
    const meta = {
        title: 'GLDT Swap App',
        description: 'GLDT Swap App Description',
    };
    const SwapInterface = dynamic(() => import('@/components/ui/swap/Swap'), {
        ssr: false,
    });
    const Transfer = dynamic(() => import('@/components/ui/transfer/Transfer'), {
        ssr: false,
    });
    return (
        <>
            <Head>
                <title>{meta.title}</title>
                <meta property={`og:title`} content={meta.title} key="title" />
                <meta property={`og:description`} content={meta.description} key="title" />
                <link rel="apple-touch-icon" sizes="180x180" href="/favicon/apple-touch-icon.png" />
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon/favicon-32x32.png" />
                <link rel="icon" type="image/png" sizes="16x16" href="/favicon/favicon-16x16.png" />
                <meta name="msapplication-TileColor" content="#da532c" />
                <meta name="theme-color" content="#ffffff" />
            </Head>
            <Layout>
                <Card
                    gridColumn={['1/13', '1/13', '2/12', '3/11', '3/11']}
                    p={cardPadding.xl}
                    position={'relative'}
                    shadow={['md', 'lg']}
                    bg="bg"
                    mx={['10px', '20px', 0, 0, 0]}
                    justifyContent={'center'}
                    gridTemplateRows={'repeat(1, 1fr)'}
                    gridTemplateColumns={'repeat(1, 1fr)'}
                    gap="3"
                    borderRadius={'2xl'}
                >
                    <Tabs
                        mt="15px"
                        variant={'enclosed'}
                        gridColumn={['1/13', '1/13', '2/12', '3/11', '3/11']}
                        position={'relative'}
                        display="grid"
                        justifyContent={'center'}
                        gridTemplateRows={'repeat(1, 1fr)'}
                        gridTemplateColumns={'repeat(1, 1fr)'}
                    >
                        <TabList display={'flex'} justifyContent={'center'}>
                            <Tab>Swap</Tab>
                            <Tab>Transfert</Tab>
                        </TabList>
                        <TabPanels>
                            <TabPanel>
                                <SwapInterface />
                            </TabPanel>
                            <TabPanel>
                                <Transfer />
                            </TabPanel>
                        </TabPanels>
                    </Tabs>
                </Card>
            </Layout>
        </>
    );
}

export default Home;
