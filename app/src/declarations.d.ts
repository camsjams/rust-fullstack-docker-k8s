declare module '*.png';
declare module '*.jpg';
declare module '*.svg';
declare let __SOME_VAR__: string;

type Car = {
	id: number;
	price: number;
	year: number;
	mileage: number;
	make: String;
	model: String;
	color: String;
	state: String;
}

type Cars = Car[];

type CarStats = {
	avgPrice: number;
	avgYear: number;
	avgMileage: number;
}
