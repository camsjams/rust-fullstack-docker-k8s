import {importWasm} from '../lib';

const rust = importWasm().catch(console.error);

const computeAverage = async (cars: Cars): Promise<CarStats> => {
	const pkg = await rust;
	if (pkg) {
		const result = pkg.stats(cars);
		return result;
	}

	return {
		price: 0,
		year: 0,
		mileage: 0
	};
};

export default computeAverage;
