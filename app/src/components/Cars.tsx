import React, {FC, useEffect, useState, useCallback, Fragment} from 'react';
import {Table, Button, ListGroup} from 'react-bootstrap';
import {getCars} from '../utils/cars';
import computeAverage from '../utils/computeAverage';

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
		<Table striped
			bordered
			hover>
			<thead>
				<tr>
					<th>id</th>
					<th>Year</th>
					<th>Make</th>
					<th>Model</th>
					<th>Price</th>
					<th>Mileage</th>
					<th>Color</th>
					<th>State</th>
				</tr>
			</thead>
			<tbody>
				{
					cars.length === 0 ?
						<tr>
							<td colSpan={8}>Loading Cars...</td>
						</tr> :
						cars.map(({id, year, make, model, price, mileage, color, state}) =>
							<tr key={id}>
								<td>{id}</td>
								<td>{year}</td>
								<td>{make}</td>
								<td>{model}</td>
								<td>{price}</td>
								<td>{mileage}</td>
								<td>{color}</td>
								<td>{state}</td>
							</tr>
						)
				}
			</tbody>
		</Table>
	</Fragment>;
};

export default Cars;
