export const formatAmount = amount => {
	const int = parseInt(amount);
	const v = int / 100000000;
	return v.toFixed(2);
};