export const getCars = (): Promise<Cars> =>
	fetch('/cars')
		.then((response) => response.json());
