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

import {
    Box,
    Button,
    Container,
    Grid,
    GridItem,
    HStack,
    Stack,
    Heading,
    Stat,
    StatGroup,
    StatLabel,
    StatNumber,
    Text,
    VStack,
    Accordion,
} from '@chakra-ui/react';
import Head from 'next/head';
import Logo from '/public/images/logo.svg';
import Yumi from '/public/images/yumi.svg';
import Image from 'next/image';
import BAS from '/public/images/partners/BAS.svg';
import dfinity from '/public/images/partners/dfinity.svg';
import KPMG from '/public/images/partners/KPMG.svg';
import loomis from '/public/images/partners/loomis.jpg';
import metalor from '/public/images/partners/metalor.svg';
import origyn from '/public/images/partners/origyn.svg';
import Link from 'next/link';
import Play from '/public/images/play.svg';
import Layout from '@/components/layout/Layout';
import Banner from '@/components/banner/Banner';
import { content } from './../src/content';
import TextBloc from '@/components/text/TextBloc';
import Partners from '@/components/partners/Partners';
import { Divider, HStack, VStack } from '@chakra-ui/react';
import FaqSection from '@/components/FAQ/FaqSection';
import Scene from '@/components/banner/scene/Scene';
import dynamic from 'next/dynamic';
import Yumi from '@/components/banner/Yumi';
import GldtValue from '@/components/stats/GldtValue';

function Home() {
    const { intro, partners, tech, price } = content;
    const meta = {
        title: 'GLDT - Home',
        description: intro,
    };
    const Stats = dynamic(() => import('@/components/stats/Stats'), {
        ssr: false,
    });
    return (
        <Layout meta={meta}>
            <VStack
                w={'100%'}
                alignItems={'flex-start'}
                spacing={['40px', '60px', '100px', '100px']}
            >
                <Banner />
                <TextBloc
                    title={intro.title}
                    content={intro.content}
                    textSpan={[12, 12, 8, 5, 5]}
                    titleSpan={[12, 12, 3, 2, 2]}
                    titleOrder={[2, 2, 2, 2]}
                    colStart={[0, 0, 0, 2, 2]}
                    colEnd={[12, 12, 12]}
                    textOrder={[2, 2, 2, 2]}
                    childrenSpan={[12, 12, 12, 4, 4]}
                >
                    <Stats />
                </TextBloc>
                <Partners />
                <TextBloc
                    title={tech.title}
                    content={tech.content}
                    link={tech.link}
                    textSpan={[12, 12, 8, 5, 5]}
                    titleSpan={[12, 12, 3, 3, 2]}
                    titleOrder={[2, 2, 2, 1]}
                    colEnd={[12, 12, 11, 11]}
                    textOrder={[2, 2, 2, -1]}
                    colStart={[0, 0, 0, 2, 2]}
                    variant={true}
                />
                <TextBloc
                    title={price.title}
                    content={price.content}
                    link={price.link}
                    subtitle={'1g gold = 100 GLDT'}
                    textSpan={[12, 12, 8, 5, 5]}
                    titleSpan={[12, 12, 3, 3, 2]}
                    titleOrder={(-2, -2, 0)}
                    colStart={[0, 0, 1, 4, 5]}
                    colEnd={[12, 12, 12, 12]}
                    textOrder={[5, 5, 2, 2, 2]}
                    titleAlign="right"
                    pastille={true}
                    circle={true}
                />
                <Yumi />
                <FaqSection full={false} />
            </VStack>
        </Layout>
    );
}
export default Home;
