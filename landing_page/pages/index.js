import {
    Box,
    Button,
    Container,
    Grid,
    GridItem,
    HStack,
    Heading,
    Stat,
    StatGroup,
    StatLabel,
    StatNumber,
    Text,
    VStack,
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

function Home({}) {
    const meta = {
        title: 'GLDT Swap',
        description: 'GLDT Swap Description',
    };

    const stats = [
        {
            label: 'USD Market Cap ',
            number: '$474 842 289',
        },
        {
            label: 'Gold Bars ',
            number: '700',
        },
        {
            label: 'Gold Kilograms',
            number: '7643.71',
        },
    ];

    const Stats = ({ label, number }) => {
        return (
            <Stat>
                <StatLabel fontSize={'24px'} fontWeight={'bold'} pb="36px" m={0}>
                    {label}
                </StatLabel>
                <StatNumber fontSize={'32px'} fontWeight={400} m={0}>
                    {number}
                </StatNumber>
            </Stat>
        );
    };

    const partners = [
        {
            name: 'dfinity',
            url: 'https://dfinity.org/',
            img: dfinity,
            w: '150px',
        },
        {
            name: 'origyn',
            url: 'https://www.origyn.com/',
            img: origyn,
            w: '200px',
        },
        {
            name: 'KPMG',
            url: 'https://kpmg.com/',
            img: KPMG,
            w: '180px',
        },
        {
            name: 'loomis',
            url: 'https://www.loomis.ch',
            img: loomis,
            w: '150px',
        },
        {
            name: 'BAS',
            url: '#',
            img: BAS,
            w: '200px',
        },
    ];
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
            <Box p={'50px'}>
                <header
                    style={{
                        display: 'flex',
                        justifyContent: 'space-between',
                    }}
                >
                    <Image width={80} src={Logo} />
                    <Button
                        _hover={{
                            bg: '#D3B872',
                        }}
                        bg="#D3B872"
                        w="161px"
                        h="80px"
                        borderRadius={'30px'}
                        fontSize={'24px'}
                        fontWeight={'bold'}
                        color={'#fff'}
                    >
                        Use GLDT
                    </Button>
                </header>
                <Container maxWidth={'1238px'} my="150px">
                    <VStack alignItems={'flex-start'} spacing={'25px'}>
                        <Heading as="h1" fontSize={'96px'} fontWeight={'bold'}>
                            GLDT – The Future of Gold Investments
                        </Heading>
                        <Heading as="h3" fontSize={'48px'} fontWeight={400}>
                            Learn how GLDT works.
                        </Heading>
                        <Button
                            w="259px"
                            h="95px"
                            fontSize={'32px'}
                            borderRadius={'30px'}
                            bg="#f4f4f4"
                            _hover={{ bg: '#f4f4f4' }}
                        >
                            <Image src={Play} width="40px" />
                            <Text ml="10px">Play video</Text>
                        </Button>
                    </VStack>
                </Container>
                <Box
                    my="150px"
                    as="section"
                    bg="#F4F4F4"
                    py="120px"
                    borderRadius={'100px 100px 0 0'}
                    mx={'-50px'}
                >
                    <Container maxWidth={'1238px'}>
                        <VStack w={'100%'} spacing={'90px'}>
                            <HStack spacing={'50px'}>
                                <VStack w="50%" spacing={'58px'}>
                                    <Heading fontSize={'48px'}>
                                        A token backed 100% by physical gold
                                    </Heading>
                                    <Text fontSize={'24px'}>
                                        GLDT is more than a token; it's the new gold standard in
                                        investment. Leveraging ICP blockchain technology, it
                                        guarantees stability akin to physical gold in a dynamic
                                        digital form. Discover transparent and secure investing with
                                        GLDT, your beacon of trust in the fluctuating world of
                                        decentralized finance.
                                    </Text>
                                </VStack>
                                <Image width={312} src={Logo} />
                            </HStack>

                            <StatGroup w={'100%'}>
                                {stats.map((e, i) => (
                                    <Stats key={i} label={e.label} number={e.number} />
                                ))}
                            </StatGroup>
                        </VStack>
                    </Container>
                </Box>
                <Box as="section" my="150px">
                    <Container maxWidth={'1238px'}>
                        <HStack spacing={'50px'} alignItems={'flex-start'}>
                            <Box>
                                <Heading fontSize={'36px'} mb="50px">
                                    Steadfast Stability, Golden Opportunity
                                </Heading>
                                <Text fontSize={'24px'}>
                                    With GLDT, enjoy the peace of mind that comes with a currency
                                    rooted in the tangible value of gold. Secure your assets and
                                    seize golden opportunities in the digital financial landscape.
                                </Text>
                            </Box>
                            <Box>
                                <Heading mb="50px" fontSize={'36px'}>
                                    Seize the Golden Standard
                                </Heading>
                                <Text fontSize={'24px'}>
                                    Invest with confidence, knowing each GLDT is backed by physical
                                    gold. Your gateway to a stable and prosperous digital economy
                                    starts with GLDT.
                                </Text>
                                <Box fontSize={'36px'} fontWeight={'bold'} mt="100px">
                                    <HStack>
                                        <Text>1g of gold = 100 GLDT</Text>
                                        <Image src={Logo} width={50} />
                                    </HStack>
                                </Box>
                            </Box>
                        </HStack>
                    </Container>
                </Box>
                <Box as="section" my="150px">
                    <Container maxWidth={'1238px'}>
                        <Heading fontSize={'36px'} textAlign={'center'} mb="57px">
                            Don’t own any GLD NFTs
                        </Heading>
                        <Box
                            display={'flex'}
                            justifyContent={'space-between'}
                            alignItems={'center'}
                            margin={'0 auto'}
                            fontSize={'36px'}
                            w={'100%'}
                            padding="61px 100px"
                            bg="#F4F4F4"
                            borderRadius={'100px'}
                        >
                            <Image src={Yumi} width={300} height="75px" />
                            <Button
                                borderRadius="30px"
                                w={'260px'}
                                h="83px"
                                fontSize={'32px'}
                                fontWeight={'bold'}
                                bg="#D3B872"
                                color={'#fff'}
                            >
                                Buy GLD NFTs
                            </Button>
                        </Box>
                    </Container>
                </Box>
                <Box as="section" my="150px">
                    <Container maxWidth={'1238px'}>
                        <Heading fontSize={'64px'} fontWeight={'light'} color="#D3B872">
                            POWERED BY
                        </Heading>
                        <HStack wrap={'wrap'} spacing={'200px'} my="75px">
                            {partners.map((e, i) => (
                                <Box w={e.w}>
                                    <Link href={e.url}>
                                        <Image src={e.img} alt={`logo ${e.name}`} />
                                    </Link>
                                </Box>
                            ))}
                        </HStack>
                    </Container>
                </Box>
                <footer></footer>
            </Box>
        </>
    );
}

export default Home;
