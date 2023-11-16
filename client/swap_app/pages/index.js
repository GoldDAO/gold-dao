/** ****************************************************************************
@file
GLDT and GLDT Swapp dApp frontend

@copyright Copyright © 2023  Bochsler Assets & Securities (BAS) SA, Switzerland
@see {@link https://bas.tech}

@license
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*******************************************************************************/

import dynamic from 'next/dynamic';
import Head from 'next/head';
import { Layout } from '@/components/layout/Layout';
import { Card, Tab, TabList, TabPanel, TabPanels, Tabs } from '@chakra-ui/react';
import { useEffect, useState } from 'react';
function Home({}) {
    const meta = {
        title: 'GLDT Swap App',
        description: 'GLDT Swap App Description',
    };
    const [isConnected, setIsConnected] = useState();

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
                <Tabs
                    mt="15px"
                    variant={'enclosed'}
                    gridColumn={['4/13', '1/13', '2/12', '3/11', '3/11']}
                    position={'static'}
                    display="grid"
                    justifyContent={'center'}
                    gridTemplateRows={'repeat(1, 1fr)'}
                    gridTemplateColumns={'repeat(1, 1fr)'}
                >
                    <TabList
                        display={'flex'}
                        justifyContent={'center'}
                        borderBottomColor={'transparent'}
                    >
                        <Tab opacity={isConnected ? 1 : 0.4}>Swap</Tab>
                        <Tab opacity={isConnected ? 1 : 0.4}>Transfer</Tab>
                    </TabList>
                    <TabPanels>
                        <TabPanel>
                            <SwapInterface setIsConnected={setIsConnected} />
                        </TabPanel>
                        <TabPanel>
                            <Transfer setIsConnected={setIsConnected} />
                        </TabPanel>
                    </TabPanels>
                </Tabs>
            </Layout>
        </>
    );
}

export default Home;
