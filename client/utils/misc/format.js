export const formatAmount = (amount, d) => {
	const int = parseInt(amount);
	const v = int / 100000000;
	return v.toFixed(d || 2) ;
};