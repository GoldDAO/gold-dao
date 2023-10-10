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
Stack;
import Play from '/public/images/play.svg';

function Home({}) {
    const meta = {
        title: 'GLDT Swap',
        description: 'GLDT Swap Description',
    };

    const margins = ['20px', '30px', '60px', '140px', '180px'];
    const headerMargins = ['20px', '30px', '49px', '49px', '49px'];

    const titleFontSize = ['40px', '60px', '60px', '80px', '96px'];
    const subtitleFontSize = ['28px', '36px', '36px', '48px', '48px'];
    const mediumFontSize = ['24px', '26px', '28px', '32px', '36px'];
    const TextSize = ['18px', '18px', '20px', '22px', '24px'];
    const buttonTextSize = ['22px', '24px', '28px', '32px', '32px'];
    const verticalSpacing = ['60px', '80px', '80px', '140px', '180px'];
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
            <Stat display={'flex'}>
                <StatLabel
                    fontSize={TextSize}
                    fontWeight={'bold'}
                    pb={['5px', '5px', '20px', '20px', '25px']}
                    m={0}
                >
                    {label}
                </StatLabel>
                <StatNumber
                    fontSize={mediumFontSize}
                    fontWeight={400}
                    m={0}
                    pb={['15px', '15px', '25px', '0', '0']}
                >
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
            <Box>
                <Box
                    pt={'34px'}
                    as="header"
                    px={headerMargins}
                    style={{
                        display: 'flex',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                    }}
                >
                    <Image width={80} src={Logo} />
                    <HStack spacing="50px">
                        <Link href="#">
                            <Text fontSize={TextSize}>FAQ</Text>
                        </Link>
                        <Button
                            as="a"
                            _hover={{
                                bg: '#D3B872',
                            }}
                            target="_blank"
                            href="https://app.gldt.org"
                            py={'40px'}
                            px="25px"
                            bg="#D3B872"
                            borderRadius={'30px'}
                            fontSize={('18px', '24px')}
                            fontWeight={'bold'}
                            color={'#fff'}
                        >
                            Use GLDT
                        </Button>
                    </HStack>
                </Box>
                <Container maxWidth={'100%'} px={margins} my={verticalSpacing}>
                    <VStack
                        alignItems={'flex-start'}
                        spacing={['5px', '15px', '15px', '25px', '25px']}
                    >
                        <Heading as="h1" fontSize={titleFontSize} fontWeight={'bold'}>
                            GLDT – The Future of Gold Investments
                        </Heading>
                        <Heading as="h3" fontSize={subtitleFontSize} fontWeight={400}>
                            Learn how GLDT works.
                        </Heading>
                        <Button
                            py={'40px'}
                            px="25px"
                            fontSize={buttonTextSize}
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
                    my={verticalSpacing}
                    as="section"
                    bg="#F4F4F4"
                    py={verticalSpacing}
                    borderRadius={[
                        '30px 30px 0 0',
                        '30px 30px 0 0',
                        '50px 50px 0 0',
                        '80px 80px 0 0',
                        '100px 100px 0 0',
                    ]}
                >
                    <Container maxWidth={'100%'} px={margins}>
                        <VStack w={'100%'} spacing={['40px', '60px', '90px']}>
                            <Stack
                                spacing={['20px', '30px', '30px', '50px', '50px']}
                                direction={['column', 'column', 'row', 'row', 'row']}
                            >
                                <VStack
                                    w={['100%', '100%', '50%', '50%', '50%']}
                                    spacing={['20px', '30px', '30px', '50px', '50px']}
                                >
                                    <Heading fontSize={subtitleFontSize}>
                                        A token backed 100% by physical gold
                                    </Heading>
                                    <Text fontSize={TextSize}>
                                        GLDT is more than a token; it&apos;s the new gold standard
                                        in investment. Leveraging ICP blockchain technology, it
                                        guarantees stability akin to physical gold in a dynamic
                                        digital form. Discover transparent and secure investing with
                                        GLDT, your beacon of trust in the fluctuating world of
                                        decentralized finance.
                                    </Text>
                                </VStack>
                                <Box display={['none', 'none', 'block', 'block', 'block']}>
                                    <Image width={312} src={Logo} />
                                </Box>
                            </Stack>
                            <HStack w="100%">
                                <StatGroup
                                    w={'100%'}
                                    display={['flex']}
                                    flexDirection={['column', 'column', 'row', 'row', 'row']}
                                >
                                    {stats.map((e, i) => (
                                        <Stats key={i} label={e.label} number={e.number} />
                                    ))}
                                </StatGroup>
                                <Box display={['block', 'block', 'none', 'none', 'none']}>
                                    <Image width={312} src={Logo} />
                                </Box>
                            </HStack>
                        </VStack>
                    </Container>
                </Box>
                <Box as="section" my={verticalSpacing}>
                    <Container maxWidth={'100%'} px={margins}>
                        <Stack
                            spacing={'50px'}
                            alignItems={'flex-start'}
                            direction={['column', 'column', 'row', 'row', 'row']}
                        >
                            <Box w="100%">
                                <Heading
                                    fontSize={mediumFontSize}
                                    mb={['20px', '30px', '30px', '50px', '50px']}
                                >
                                    Steadfast Stability, Golden Opportunity
                                </Heading>
                                <Text fontSize={TextSize}>
                                    With GLDT, enjoy the peace of mind that comes with a currency
                                    rooted in the tangible value of gold. Secure your assets and
                                    seize golden opportunities in the digital financial landscape.
                                </Text>
                            </Box>
                            <Box w="100%">
                                <Heading
                                    mb={['20px', '30px', '30px', '50px', '50px']}
                                    fontSize={mediumFontSize}
                                >
                                    Seize the Golden Standard
                                </Heading>
                                <Text fontSize={TextSize}>
                                    Invest with confidence, knowing each GLDT is backed by physical
                                    gold. Your gateway to a stable and prosperous digital economy
                                    starts with GLDT.
                                </Text>
                            </Box>
                        </Stack>
                        <Box
                            fontSize={mediumFontSize}
                            fontWeight={'bold'}
                            mt={['40px', '60px', '60px', '100px', '100px']}
                        >
                            <Stack
                                alignItems={[
                                    'center',
                                    'center',
                                    'center',
                                    'space-between',
                                    'space-between',
                                ]}
                                w="100%"
                                spacing={'50px'}
                                direction={['column', 'column', 'column', 'row', 'row']}
                            >
                                <Box w="100%">
                                    <Stack
                                        w={['100%', '100%', '100%', 'fit-content', 'fit-content']}
                                        justifyContent="center"
                                    >
                                        <Button
                                            bg="#F4F4F4"
                                            fontSize={buttonTextSize}
                                            borderRadius={30}
                                            p={'40px'}
                                            _hover={{
                                                bg: '#f4f4f4',
                                            }}
                                        >
                                            Read the whitepaper
                                        </Button>
                                    </Stack>
                                </Box>
                                <HStack
                                    w="100%"
                                    justifyContent={[
                                        'center',
                                        'center',
                                        'center',
                                        'flex-start',
                                        'flex-start',
                                    ]}
                                >
                                    <Text>1g of gold = 100</Text>
                                    <HStack>
                                        <Text>GLDT</Text> <Image src={Logo} width={50} />
                                    </HStack>
                                </HStack>
                            </Stack>
                        </Box>
                    </Container>
                </Box>
                <Box as="section" my={verticalSpacing}>
                    <Container maxWidth={'100%'} px={margins}>
                        <Heading
                            fontSize={mediumFontSize}
                            textAlign={'center'}
                            mb={['20px', '30px', '30px', '40px', '57px']}
                        >
                            Don’t own any GLD NFTs
                        </Heading>
                        <Box
                            maxW={'824px'}
                            wrap="wrap"
                            display={'flex'}
                            flexDirection={['column', 'column', 'row', 'row', 'row']}
                            justifyContent={'space-between'}
                            alignItems={'center'}
                            margin={'0 auto'}
                            fontSize={'36px'}
                            w={'100%'}
                            padding="61px 100px"
                            bg="#F4F4F4"
                            borderRadius={['30px', '30px', '50px', '80px', '100px ']}
                        >
                            <Image src={Yumi} width={300} height="75px" />
                            <Button
                                as="a"
                                href="https://yumi.io/gold"
                                target="_blank"
                                mt={['20px', '20px', '20px', '0', '0']}
                                borderRadius="30px"
                                w={'260px'}
                                h="83px"
                                _hover={{
                                    backgroundColor: '#D3B872',
                                }}
                                fontSize={buttonTextSize}
                                fontWeight={'bold'}
                                bg="#D3B872"
                                color={'#fff'}
                            >
                                Buy GLD NFTs
                            </Button>
                        </Box>
                    </Container>
                </Box>
                <Box as="section" my={verticalSpacing}>
                    <Container maxWidth={'100%'} px={margins}>
                        <Heading
                            fontSize={['36px', '36px', '36px', '48px', '64px']}
                            fontWeight={'light'}
                            color="#D3B872"
                        >
                            POWERED BY
                        </Heading>
                        <HStack
                            wrap={'wrap'}
                            spacing={['60px', '100px', '100px', '150px', '200px']}
                            my={['50px', '75px']}
                            justifyContent={'center'}
                        >
                            {partners.map((e, i) => (
                                <Box w={['100px', '150px', e.w]} key={i}>
                                    <Link href={e.url}>
                                        <Image src={e.img} alt={`logo ${e.name}`} />
                                    </Link>
                                </Box>
                            ))}
                        </HStack>
                    </Container>
                </Box>
                <Box
                    as="footer"
                    style={{
                        width: '100%',
                        backgroundColor: '#F4F4F4',
                    }}
                    h={['250px', '300px', '400px']}
                ></Box>
            </Box>
        </>
    );
}

export default Home;
