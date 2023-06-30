import { join } from 'path'
import fs from 'fs'
import matter from 'gray-matter'

export const getMarkdownPage = pageName => {
    const contentDirectory = join(process.cwd(), `src/content/pages/${pageName}.md`)
    const fileContents = fs.readFileSync(contentDirectory, 'utf8')
    const { data, content } = matter(fileContents)
    return {data: data, content: content}
}

export const getPartners = async () => {
    const partnersArray = []
    const contentDirectory = join(process.cwd(), `src/content/components/partners`)
    fs.readdirSync(contentDirectory).forEach(file => {
        const contentDirectory = join(process.cwd(), `src/content/components/partners/${file}`)
        const fileContents = fs.readFileSync(contentDirectory, 'utf8')
        const { data } = matter(fileContents)
        partnersArray.push(data)
    });
    return partnersArray
}

export const getSwapCTO = () => {
    const contentDirectory = join(process.cwd(), `src/content/components/swapCTO/content.md`)
    const fileContents = fs.readFileSync(contentDirectory, 'utf8')
    const { data, content } = matter(fileContents)
    return {data: data, content: content}
}

export const getMarketCap = () => {
    const contentDirectory = join(process.cwd(), `src/content/components/marketcap/content.md`)
    const fileContents = fs.readFileSync(contentDirectory, 'utf8')
    const { data, content } = matter(fileContents)
    return {data: data, content: content}
}