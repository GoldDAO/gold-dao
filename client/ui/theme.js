export const customTheme = {
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
				},
				h1: {
					fontSize: '56px',
					fontWeight: 500,
					color: 'gold',
				},
				h2: {
					fontSize: '56px',
					fontWeight: 400,
				}
			}
		},
		Text: {
			baseStyle: {
				fontSize: '20px',
				lineheight: '26px',
				fontWeight: 400,
			}
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
				},
				yumiInverted: {
					bg: 'white',
					borderColor: 'black',
					border: '1px',
					color: 'black',
					borderRadius: '50px',
					fontSize: '18px',
					fontWeight: 400,
				}
			}
		}
	},
};

export const cardPadding = {
	xl: [2, 2, 2, 4, 4],
	lg: [2, 2, 2, 4, 4],
	md: [2, 2, 2, 4, 4],
	sm: [2, 2, 2, 4, 4],
	xs: [2, 2, 2, 4, 4],
};
