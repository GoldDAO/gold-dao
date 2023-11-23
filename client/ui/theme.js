import { Inter } from 'next/font/google';

const inter = Inter({
	subsets: ['latin'],
	weight: ['300', '400', '500'],
	fallback: ['system-ui', 'arial', 'sans-serif'],
});

export const customTheme = {
	fonts: {
		heading: inter.style.fontFamily,
		body: inter.style.fontFamily,
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
		veryDarkGold: '#7a6d49'
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
					fontSize: ['46px' ,'56px', '60px', '60px', '74px', ],
					lineHeight:[ '48px', '60px', '68px', '68px','80px', ],
					fontWeight: 500,
					color: 'gold',
					fontFamily: inter.style.fontFamily ,
				},
				h2: {
					fontSize: ['46px' ,'56px', '60px', '60px','74px', ],
					lineHeight:[ '48px', '60px', '68px', '68px','80px',],
					fontWeight: 500,
					fontFamily: inter.style.fontFamily ,
				}
			}
		},
		Input: {
			defaultProps: {
				focusBorderColor: 'black',
			}
		},
		NumberInput: {
			defaultProps: {
				focusBorderColor: 'black',
			}
		},
		Text: {
			baseStyle: {
				fontSize: ['18px', '18px','20px'],
				lineheight: ['22px', '22px','26px'],
				fontWeight: 400,
				fontFamily: inter.style.fontFamily
			},

		},
		Alert: {
			baseStyle: {
				fontFamily: inter.style.fontFamily,
				container: {
					bg: 'white',
					color: 'black',
					borderRadius: '140px',
					border: '1px',
					borderColor: 'black',
				},
				title: {
					color: 'black',
					fontWeight: 500,
				},
				description: {
					color: 'black',
					fontWeight:300,

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
					},
				},
				disableYumiGold: {
					bg: 'bg',
					border: 'none',
					outline: 'none',
					color: 'secondaryText',
					borderRadius: '50px',
					fontSize: '18px',
					fontWeight: 500,
				}
			}
		},
	},
};

export const cardPadding = {
	xl: [2, 3, 4, 4, 4],
	lg: [2, 2, 2, 4, 4],
	md: [2, 2, 2, 4, 4],
	sm: [2, 2, 2, 4, 4],
	xs: [2, 2, 2, 4, 4],
};
