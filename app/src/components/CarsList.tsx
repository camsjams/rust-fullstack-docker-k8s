import React, {FC} from 'react';
import {Table} from 'react-bootstrap';

type Props = {
	cars: Cars;
};

const CarsList: FC<Props> = ({cars}) =>
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
	</Table>;

export default CarsList;
