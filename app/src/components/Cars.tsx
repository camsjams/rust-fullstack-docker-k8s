import React, {FC, useEffect, useState, useCallback, Fragment} from 'react';
import {Button, ListGroup} from 'react-bootstrap';
import {getCars} from '../utils/cars';
import computeAverage from '../utils/computeAverage';
import CarsList from './CarsList';

const Cars: FC = () => {
	const [cars, setCars] = useState([] as Cars);
	useEffect((): void => {
		getCars()
			.then((cars: Cars) => setCars(cars))
			.catch((error) => console.error('Error Fetching Cars:', error));
	}, []);

	const [crunchedNumbers, setCrunched] = useState(undefined as CarStats);
	const handleCrunchClick = useCallback((): void => {
		computeAverage(cars)
			.then((result: CarStats) => setCrunched(result))
			.catch((error) => console.error('Error Crunching Cars:', error));
	}, [crunchedNumbers, cars]);

	return <Fragment>
		<p>
			<Button
				variant="secondary"
				onClick={handleCrunchClick}>
				Compute Averages with WASM
			</Button>
			{
				crunchedNumbers ?
					<ListGroup>
						<ListGroup.Item>Average Price: {crunchedNumbers.price}</ListGroup.Item>
						<ListGroup.Item>Average Year: {crunchedNumbers.year}</ListGroup.Item>
						<ListGroup.Item>Average Mileage: {crunchedNumbers.mileage}</ListGroup.Item>
					</ListGroup> :
					null
			}
		</p>
		<CarsList cars={cars} />
	</Fragment>;
};

export default Cars;
