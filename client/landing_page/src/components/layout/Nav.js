import {
    Box,
    Flex,
    Text,
    IconButton,
    Stack,
    VStack,
    Collapse,
    Icon,
    Popover,
    PopoverTrigger,
    PopoverContent,
    useColorModeValue,
    useDisclosure,
    HStack,
    Button,
    GridItem,
} from '@chakra-ui/react';
import { HamburgerIcon, CloseIcon, ChevronDownIcon, ChevronRightIcon } from '@chakra-ui/icons';
import Link from 'next/link';
import Logo from '/public/images/logo.svg';
import Image from 'next/image';
import React from 'react';

export default function WithSubnavigation({ nav, children }) {
    const { isOpen, onToggle } = useDisclosure();

    return (
        <Box>
            <Flex align={'center'}>
                <Flex
                    flex={{ base: 1, md: 'auto' }}
                    display={{ base: 'flex', md: 'none' }}
                    justify={'flex-start'}
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
                <Flex
                    display={{ base: 'none', md: 'flex' }}
                    flex={{ base: 1 }}
                    justify={{ base: 'center', md: 'start' }}
                >
                    <Flex
                        display={{ base: 'none', md: 'flex' }}
                        justifyContent={'space-between'}
                        w="100%"
                    >
                        <DesktopNav nav={nav}> {children}</DesktopNav>
                    </Flex>
                </Flex>
            </Flex>
            <Collapse in={isOpen} animateOpacity>
                <MobileNav nav={nav}>{children}</MobileNav>
            </Collapse>
        </Box>
    );
}

const DesktopNav = ({ nav, children }) => {
    const linkColor = useColorModeValue('gray.600', 'gray.200');
    const linkHoverColor = useColorModeValue('gray.800', 'white');
    const popoverContentBgColor = useColorModeValue('white', 'gray.800');

    return (
        <Stack
            direction={'row'}
            alignItems={'center'}
            w={'100%'}
            justifyContent={'space-between'}
            className="asf"
        >
            {nav.map((navItem) => (
                <Box key={navItem.label}>
                    <Popover trigger={'hover'} placement={'bottom-start'}>
                        <PopoverTrigger>
                            <Link href={navItem.href}>
                                <Text
                                    transition=".2s all"
                                    fontSize={'18px'}
                                    color="back"
                                    _hover={{ textDecoration: 'underline', opacity: 0.7 }}
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
                                <Stack alignItems={'center'}>
                                    {navItem.children.map((child) => (
                                        <DesktopSubNav key={child.label} {...child} />
                                    ))}
                                </Stack>
                            </PopoverContent>
                        )}
                    </Popover>
                </Box>
            ))}
            {children}
            <Button width={'150px'} variant={'yumiGold'}>
                Swap
            </Button>
        </Stack>
    );
};

const DesktopSubNav = ({ label, href, subLabel }) => {
    return (
        <Box
            as="a"
            href={href}
            role={'group'}
            p={2}
            display="flex"
            alignItems={'center'}
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

const MobileNav = ({ nav, children }) => {
    return (
        <VStack p={4} display={{ base: 'flex', md: 'none' }} justifyContent={'flex-start'}>
            <Box width={'100%'}>
                {nav.map((navItem) => (
                    <MobileNavItem key={navItem.label} {...navItem} />
                ))}
            </Box>
            {children}
            <Button variant={'yumiGold'} width={'100%'}>
                {' '}
                Swap
            </Button>
        </VStack>
    );
};

const MobileNavItem = ({ label, children, href }) => {
    const { isOpen, onToggle } = useDisclosure();

    return (
        <Stack spacing={4} onClick={children && onToggle}>
            <Link href={href}>
                <Text
                    transition=".2s all"
                    color="back"
                    _hover={{ textDecoration: 'underline', opacity: 0.7 }}
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
