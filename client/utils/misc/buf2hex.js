export const buf2hex = (buffer) => {
	return [...new Uint8Array(buffer)].map((x) => x.toString(16).padStart(2, '0')).join('');
};

export const  stringToUint8Array = (inputString) => {
	if (inputString.length !== 64) {
		console.log('Hex string must represent 32 bytes of data');
	}

	const byteArray = new Uint8Array(32);

	for (let i = 0, j = 0; i < inputString.length; i += 2, j++) {
		byteArray[j] = parseInt(inputString.substring(i, i + 2), 16);
	}

	return byteArray;
};