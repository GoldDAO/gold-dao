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
import { useRouter } from 'next/router';

import Metas from '@ui/layout/Metas';

function Home({}) {
    const router = useRouter();
    const meta = {
        title: 'GLDT Explorer',
        description: 'GLDT Explorer Description',
    };
    if (router.asPath.includes("transaction/")) {
        console.debug(`Routing to ${router.basePath}${router.asPath} !`);
        router.push(router.asPath);
    }
    const Explorer = dynamic(() => import('@/components/explorer/Explorer'), {
        ssr: false,
    });

    return (
        <>
            <Metas meta={meta} />
            <Layout>
                <Explorer />
            </Layout>
        </>
    );
}

export default Home;
