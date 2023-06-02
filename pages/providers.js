import React from 'react';
import { getMarkdownPage } from '../lib/utils/getMarkdown';
import { markdownToHtml } from '../lib/utils/markdownToHtml';
import { PageContent } from './_app';
import { ConnectButton, ConnectDialog } from '@connect2ic/react';
import Layout from '../lib/components/UI/layout/Layout';

const Providers = ({ meta, content }) => {
    return (
        <Layout meta={{ title: meta.title, description: meta.description }}>
            <PageContent dangerouslySetInnerHTML={{ __html: content }} />
            {/* <ProviderList /> */}
            <ConnectButton />
            <ConnectDialog dark={false} />
        </Layout>
    );
};

export default Providers;


export async function getStaticProps() {
    const content = getMarkdownPage('providers')
    const html = await markdownToHtml(content.content)
    return {
        props: {
            content: html,
            meta: content.data,
        }
    }
}
