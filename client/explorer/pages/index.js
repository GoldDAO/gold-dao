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

import Layout from './../src/components/layout/Layout';
import dynamic from 'next/dynamic';
import Head from 'next/head';

function Home({}) {
    const meta = {
        title: 'GLDT Explorer',
        description: 'GLDT Explorer Description',
    };
    const Explorer = dynamic(() => import('@/components/explorer/Explorer'), {
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
                <Explorer />
            </Layout>
        </>
    );
}

export default Home;
