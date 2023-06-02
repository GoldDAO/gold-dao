import Head from "next/head";
import Layout from "../lib/components/UI/layout/Layout";
import { getMarkdownPage, getMarketCap, getPartners, getSwapCTO } from "../lib/utils/getMarkdown";
import { markdownToHtml } from "../lib/utils/markdownToHtml";
import SwapCTO from "../lib/components/UI/SwapCTO";
import Marketcap from "../lib/components/UI/Marketcap";
import { PageContent } from "./_app";
import Partners from "../lib/components/UI/partners";
import Dialog from "../lib/components/UI/layout/Dialog";
import NFTsTable from "../lib/components/UI/layout/table/NFTsTable";
import { useState } from "react";

function HomePage({ content, meta, partners, cto, marketcap }) {
	const [openCTO, setOpenCTO] = useState(false)

	return (
		<>
			<Head>
				<title>{meta.title}</title>
				<meta property={`og:title`} content={meta.title} key="title" />
				<meta property={`og:description`} content={meta.description} key="title" />
			</Head>
			<Layout>
				<Dialog
					title="Select your GLD NFT(s) you want to swap for GLDT"
					address="address"
					open={openCTO}
					setOpen={setOpenCTO}
					content={<NFTsTable />} />
				<PageContent dangerouslySetInnerHTML={{ __html: content }} />
				<SwapCTO open={openCTO} setOpen={setOpenCTO} data={cto.data} />
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
			marketcap: marketcap,
		}
	}
}

