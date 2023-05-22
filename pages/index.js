import Head from "next/head";
import Layout from "../lib/layout/Layout";
import {remark} from 'remark'
import { getMarkdownPage, getMarketCap, getPartners, getSwapCTO } from "../lib/utils/getMarkdown";
import { markdownToHtml } from "../lib/utils/markdownToHtml";
import { Box } from "@mui/system";
import Image from "next/image";
import Link from "next/link";
import SwapCTO from "../lib/ components/SwapCTO";
import Marketcap from "../lib/ components/Marketcap";

function HomePage({content, meta, partners, cto, marketcap}) {
	return (
		<>
			<Head>
				<title>{meta.title}</title>
				<meta property={`og:title`} content={meta.title} key="title" />			
				<meta property={`og:description`} content={meta.description} key="title" />			
			</Head>
			<Layout>
				<Box dangerouslySetInnerHTML={{__html: content}}></Box>
				{partners.map((e,i) => (
					<Box key={i}>
						<Link href={e.url} target="_blank" referrerPolicy="noreferrer">
							<Image width={200} height={100} src={e.logo} alt={`${e.name}'s logo`} />
						</Link>
					</Box>
				))}
				<SwapCTO data={cto.data}/>
				<Marketcap data={marketcap.data} />
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