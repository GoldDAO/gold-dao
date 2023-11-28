import {
	Box,
	Flex,
	Text,
	IconButton,
	Stack,
	Collapse,
	Icon,
	Popover,
	PopoverTrigger,
	PopoverContent,
	useColorModeValue,
	useDisclosure,
	HStack,
} from '@chakra-ui/react';
import { HamburgerIcon, CloseIcon,  ChevronRightIcon } from '@chakra-ui/icons';
import C2icButton from '@/components/c2ic/C2icButton';
import Link from 'next/link';
import Logo from '@ui/assets/logo.svg';
import Image from 'next/image';
import { useConnect } from '@connect2ic/react';
import React from 'react';

export default function WithSubnavigation({nav, children, noConnection}) {
	const { isOpen, onToggle } = useDisclosure();
	const { isConnected, principal } = useConnect();

	return (
		<Box mt="15px" >
			<Flex align={'center'} >
				<Flex
					flex={{ base: 1, md: 'auto' }}
					ml={{ base: -2 }}
					display={{ base: 'flex', md: 'none' }}
				>
					{isConnected && (
						<IconButton
							onClick={onToggle}
							icon={
								isOpen ? <CloseIcon w={3} h={3} /> : <HamburgerIcon w={5} h={5} />
							}
							variant={'ghost'}
							aria-label={'Toggle Navigation'}
							_hover={{
								bg: 'extraLightGold',
							}}
						/>
					)}
				</Flex>
				<Flex flex={{ base: 1 }} justify={{ base: 'center', md: 'start' }}>
					<Link href="/">
						<Image src={Logo} width={50} height={50} alt="gldt-token-logo" />
					</Link>

					<Flex
						display={{ base: 'none', md: 'flex' }}
						ml={10}
						w={'100%'}
						justifyContent={'space-between'}
						
					>
						{isConnected && <DesktopNav nav={nav}> {children}</DesktopNav>}
					</Flex>
				</Flex>
				<Stack flex={{ base: 1, md: 0 }} justify={'flex-end'} direction={'row'} spacing={6}>
					<C2icButton />
				</Stack>
			</Flex>

			<Collapse in={isOpen} animateOpacity>
				{isConnected && <MobileNav nav={nav}>{children}</MobileNav>}
			</Collapse>
		</Box>
	);
}

const DesktopNav = ({nav, children}) => {
	const linkColor = useColorModeValue('gray.600', 'gray.200');
	const linkHoverColor = useColorModeValue('gray.800', 'white');
	const popoverContentBgColor = useColorModeValue('white', 'gray.800');

	return (
		<Stack
			direction={'row'}
			spacing={20}
			alignItems={'center'}
			justifyContent={'space-between'}
			w={'100%'}
			pr={['20px', '20px', '20px', '40px']}
		>
			{children}
			{nav.map((navItem) => (
				<Box key={navItem.label}  >
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

const MobileNav = ({nav, children}) => {
	return (
		<HStack p={4} display={{ base: 'flex', md: 'none' }} justifyContent={'space-between'}>
			<Box>
				{nav.map((navItem) => (
					<MobileNavItem key={navItem.label} {...navItem} />
				))}
			</Box>
			{children}
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
