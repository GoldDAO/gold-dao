import Head from "next/head";
import Layout from "../lib/layout/Layout";
import { getMarkdownPage, getMarketCap, getPartners, getSwapCTO } from "../lib/utils/getMarkdown";
import { markdownToHtml } from "../lib/utils/markdownToHtml";
import SwapCTO from "../lib/ components/SwapCTO";
import Marketcap from "../lib/ components/Marketcap";
import { PageContent } from "./_app";
import Partners from "../lib/ components/partners";

function HomePage({content, meta, partners, cto, marketcap}) {
	return (
		<>
			<Head>
				<title>{meta.title}</title>
				<meta property={`og:title`} content={meta.title} key="title" />			
				<meta property={`og:description`} content={meta.description} key="title" />			
			</Head>
			<Layout>
				<PageContent dangerouslySetInnerHTML={{__html: content}} />
				<SwapCTO data={cto.data}/>
				<Marketcap data={marketcap.data} />
				<Partners partners={partners} />
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
			marketcap: marketcap
		}
	}
  }

