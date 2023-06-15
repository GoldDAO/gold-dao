import Head from "next/head";
import Layout from "../lib/components/UI/layout/Layout";
import { getMarkdownPage, getMarketCap, getPartners, getSwapCTO } from "../lib/utils/getMarkdown";
import { markdownToHtml } from "../lib/utils/markdownToHtml";
import SwapContainer from "../lib/components/UI/sequence/SwapContainer";
import Marketcap from "../lib/components/UI/Marketcap";
import { PageContent } from "./_app";
import Partners from "../lib/components/UI/partners";
import BatchOffers from "../lib/components/commands/batchOffers";

function HomePage({ content, meta, partners, cto, marketcap }) {
	return (
		<>
			<Head>
				<title>{meta.title}</title>
				<meta property={`og:title`} content={meta.title} key="title" />
				<meta property={`og:description`} content={meta.description} key="title" />
			</Head>
			<Layout>
				<PageContent dangerouslySetInnerHTML={{ __html: content }} />
				<SwapContainer data={cto.data} />
				<Marketcap data={marketcap.data} />
				<Partners partners={partners} />
				<BatchOffers />
			</Layout>
		</>
	)
}

export default HomePage;


export async function getStaticProps() {

	const content = getMarkdownPage('home')
	const html = await markdownToHtml(content.content)
	const partners = await getPartners()
	const CTO = await getSwapCTO()
	const marketcap = await getMarketCap()
	return {
		props: {
			content: html,
			meta: content.data,
			partners: partners,
			cto: CTO,
			marketcap: marketcap,
		}
	}
}

