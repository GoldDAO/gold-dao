import {
    Box,
    Flex,
    Text,
    IconButton,
    Button,
    Stack,
    Collapse,
    Icon,
    Popover,
    PopoverTrigger,
    PopoverContent,
    useColorModeValue,
    useBreakpointValue,
    useDisclosure,
    Grid,
    GridItem,
    HStack,
} from '@chakra-ui/react';
import { HamburgerIcon, CloseIcon, ChevronDownIcon, ChevronRightIcon } from '@chakra-ui/icons';
import C2icButton from '@/components/c2ic/C2icButton';
import Link from 'next/link';
import Logo from '/public/images/logo.svg';
import Image from 'next/image';
import Principal from './Principal';
import Balance from './Balance';
import { useConnect } from '@connect2ic/react';

export default function WithSubnavigation() {
    const { isOpen, onToggle } = useDisclosure();
    const { isConnected, principal } = useConnect();

    return (
        <Box mt="15px">
            <Flex minH={'60px'} py={{ base: 2 }} px={{ base: 4 }} align={'center'}>
                <Flex
                    flex={{ base: 1, md: 'auto' }}
                    ml={{ base: -2 }}
                    display={{ base: 'flex', md: 'none' }}
                >
                    <IconButton
                        onClick={onToggle}
                        icon={isOpen ? <CloseIcon w={3} h={3} /> : <HamburgerIcon w={5} h={5} />}
                        variant={'ghost'}
                        aria-label={'Toggle Navigation'}
                        _hover={{
                            bg: 'extraLightGold',
                        }}
                    />
                </Flex>
                <Flex flex={{ base: 1 }} justify={{ base: 'center', md: 'start' }}>
                    <Link href="/">
                        <Image src={Logo} width={50} height={50} alt="gldt-token-logo" />
                    </Link>

                    <Flex
                        display={{ base: 'none', md: 'flex' }}
                        ml={10}
                        justifyContent={'space-between'}
                    >
                        {isConnected && <DesktopNav />}
                    </Flex>
                </Flex>
                <Stack flex={{ base: 1, md: 0 }} justify={'flex-end'} direction={'row'} spacing={6}>
                    <C2icButton />
                </Stack>
            </Flex>

            <Collapse in={isOpen} animateOpacity>
                {isConnected && <MobileNav />}
            </Collapse>
        </Box>
    );
}

const Wallet = () => {
    return (
        <Grid>
            <GridItem gridColumn={'span 3'}>
                <Principal />
            </GridItem>
            <GridItem gridColumn={'span 3'}>
                <Balance />
            </GridItem>
        </Grid>
    );
};

const DesktopNav = () => {
    const linkColor = useColorModeValue('gray.600', 'gray.200');
    const linkHoverColor = useColorModeValue('gray.800', 'white');
    const popoverContentBgColor = useColorModeValue('white', 'gray.800');

    return (
        <Stack
            direction={'row'}
            spacing={20}
            alignItems={'center'}
            justifyContent={'space-between'}
            pr={['20px', '20px', '20px', '40px']}
        >
            {NAV_ITEMS.map((navItem) => (
                <Box key={navItem.label}>
                    <Popover trigger={'hover'} placement={'bottom-start'}>
                        <PopoverTrigger>
                            <Link href={navItem.href}>
                                <Text
                                    color="darkGold"
                                    transition=".2s all"
                                    _hover={{ textDecoration: 'underline', color: 'gold' }}
                                    p={2}
                                    fontSize={'sm'}
                                    fontWeight={500}
                                >
                                    {navItem.label}
                                </Text>
                            </Link>
                        </PopoverTrigger>

                        {navItem.children && (
                            <PopoverContent
                                border={0}
                                boxShadow={'xl'}
                                bg={popoverContentBgColor}
                                p={4}
                                rounded={'xl'}
                                minW={'sm'}
                            >
                                <Stack>
                                    {navItem.children.map((child) => (
                                        <DesktopSubNav key={child.label} {...child} />
                                    ))}
                                </Stack>
                            </PopoverContent>
                        )}
                    </Popover>
                </Box>
            ))}
            <Wallet />
        </Stack>
    );
};

const DesktopSubNav = ({ label, href, subLabel }) => {
    return (
        <Box
            as="a"
            href={href}
            role={'group'}
            display={'block'}
            p={2}
            rounded={'md'}
            _hover={{ bg: useColorModeValue('pink.50', 'gray.900') }}
        >
            <Stack direction={'row'} align={'center'}>
                <Box>
                    <Text
                        transition={'all .3s ease'}
                        _groupHover={{ color: 'pink.400' }}
                        fontWeight={500}
                    >
                        {label}
                    </Text>
                    <Text fontSize={'sm'}>{subLabel}</Text>
                </Box>
                <Flex
                    transition={'all .3s ease'}
                    transform={'translateX(-10px)'}
                    opacity={0}
                    _groupHover={{ opacity: '100%', transform: 'translateX(0)' }}
                    justify={'flex-end'}
                    align={'center'}
                    flex={1}
                >
                    <Icon color={'pink.400'} w={5} h={5} as={ChevronRightIcon} />
                </Flex>
            </Stack>
        </Box>
    );
};

const MobileNav = () => {
    return (
        <HStack p={4} display={{ base: 'flex', md: 'none' }} justifyContent={'space-between'}>
            <Box>
                {NAV_ITEMS.map((navItem) => (
                    <MobileNavItem key={navItem.label} {...navItem} />
                ))}
            </Box>
            <Wallet />
        </HStack>
    );
};

const MobileNavItem = ({ label, children, href }) => {
    const { isOpen, onToggle } = useDisclosure();

    return (
        <Stack spacing={4} onClick={children && onToggle}>
            <Link href={href}>
                <Text
                    color="darkGold"
                    transition=".2s all"
                    _hover={{ textDecoration: 'underline', color: 'gold' }}
                    p={2}
                    fontSize={'md'}
                    fontWeight={500}
                >
                    {label}
                </Text>
            </Link>

            <Collapse in={isOpen} animateOpacity style={{ marginTop: '0!important' }}>
                <Stack
                    mt={2}
                    pl={4}
                    borderLeft={1}
                    borderStyle={'solid'}
                    borderColor={useColorModeValue('gray.200', 'gray.700')}
                    align={'start'}
                >
                    {children &&
                        children.map((child) => (
                            <Box as="a" key={child.label} py={2} href={child.href}>
                                {child.label}
                            </Box>
                        ))}
                </Stack>
            </Collapse>
        </Stack>
    );
};

const NAV_ITEMS = [
    {
        label: 'My Account',
        href: '/my-account',
    },
];
