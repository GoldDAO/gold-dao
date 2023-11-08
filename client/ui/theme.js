import { Inter } from 'next/font/google';

const inter = Inter({
	subsets: ['latin'],
	weight: ['300', '400', '500'],
	fallback: ['system-ui', 'arial', 'sans-serif'],
});

export const customTheme = {
	fonts: {
		inter: inter.style.fontFamily
	},
	colors: {
		transparent: 'transparent',
		black: '#000',
		white: '#fff',
		bg: '#F6F6F6',
		disableBg: '#FCFCFC',
		disableText: '#CCCCCC',
		secondaryText: '#9A9A9A',
		lightGold: '#F9E4BF',
		extraLightGold: '#F6F0DE',
		gold: '#D3B872',
		lightBlack: '#2E2E2E',
		border: '#E4E4E4',
		darkGold: '#937937',
	},
	components: {
		Tooltip: {
			baseStyle: {
				bg: 'white',
				color: 'black',
				border: '1px',
				borderColor: 'black',
				borderRadius: '20px',
				padding: '10px 15px',
			},
		},
		Heading: {
			variants: {
				h4: {
					fontSize: '20px',
					lineHeight: '26px',
					fontWeight: 500,
					fontFamily: inter.style.fontFamily ,
				},
				h1: {
					fontSize: ['42px' ,'56px', '56px'],
					fontWeight: 500,
					color: 'gold',
					fontFamily: inter.style.fontFamily ,
				},
				h2: {
					fontSize: ['42px' ,'56px', '56px'],
					fontWeight: 500,
					fontFamily: inter.style.fontFamily ,
				}
			}
		},
		Text: {
			baseStyle: {
				fontSize: ['18px', '18px','20px'],
				lineheight: ['22px', '22px','26px'],
				fontWeight: 400,
			},

		},
		Alert: {
			baseStyle: {
				container: {
					bg: 'white',
					color: 'black',
					borderRadius: '40px',
					border: '1px',
					borderColor: 'black',
				},
				title: {
					color: 'black',
				},
				description: {
					color: 'black',
				},
				icon: {
					color: 'black',
				},
			},
		},
		Button: {
			variants: {
				yumi: {
					bg: 'black',
					color: 'white',
					borderRadius: '50px',
					fontSize: '18px',
					fontWeight: 400,
					_hover: {
						transform: 'scale(1.1)'
					}
				},
				yumiGold: {
					bg: 'gold',
					border: 'none',
					outline: 'none',
					color: 'white',
					borderRadius: '50px',
					fontSize: '18px',
					fontWeight: 500,
					_hover: {
						transform: 'scale(1.1)'
					}
				}
			}
		},
	},
};

export const cardPadding = {
	xl: [2, 2, 2, 4, 4],
	lg: [2, 2, 2, 4, 4],
	md: [2, 2, 2, 4, 4],
	sm: [2, 2, 2, 4, 4],
	xs: [2, 2, 2, 4, 4],
};
