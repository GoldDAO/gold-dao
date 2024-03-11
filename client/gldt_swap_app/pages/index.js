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
import Metas from '@ui/layout/Metas';

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

    const tabSelected = {
        color: '#fff',
        bg: 'black',
    };

    return (
        <>
            <Metas meta={meta} />
            <Layout>
                <Tabs
                    mt="15px"
                    variant="soft-rounded"
                    colorScheme="black"
                    gridColumn={['1/13', '1/13', '2/12', '3/11', '4/10']}
                    position={'static'}
                    display="grid"
                    justifyContent={'center'}
                    gridTemplateRows={'repeat(1, 1fr)'}
                    gridTemplateColumns={'repeat(1, 1fr)'}
                >
                    <TabList
                        display={'flex'}
                        bg="bg"
                        w={'fit-content'}
                        alignSelf={'center'}
                        justifyContent={'center'}
                        borderRadius="50px"
                        borderBottomColor={'transparent'}
                        justifySelf={'center'}
                        opacity={isConnected ? 1 : 0.4}
                        py="3px"
                    >
                        <Tab
                            color={'blackAlpha.600'}
                            _selected={tabSelected}
                            mx="2px"
                            fontSize={'16px'}
                            fontWeight={500}
                        >
                            Swap
                        </Tab>
                        <Tab
                            color={'blackAlpha.600'}
                            _selected={tabSelected}
                            mx="2px"
                            fontWeight={500}
                            fontSize={'16px'}
                        >
                            Transfer
                        </Tab>
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
