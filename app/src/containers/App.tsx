import React, {FC, Fragment, useCallback, useState} from 'react';
import styled from 'styled-components';
import {Container, Jumbotron, Button} from 'react-bootstrap';
import Nav from '../components/Nav';
import Cars from '../components/Cars';
import GlobalStyle from '../styles/GlobalStyle';

const StyledJumbotron = styled(Jumbotron)`
	margin-top: 4em;
`;

const App: FC = () => {
	const [isShowingCars, setIsShowingCars] = useState(false);
	const handleCarsClick = useCallback((): void => {
		setIsShowingCars(!isShowingCars);
	}, [isShowingCars]);

	return <Fragment>
		<GlobalStyle />
		<Nav />
		<Container as="main">
			<StyledJumbotron>
				<h1>Rust Fullstack</h1>
				<p>
					Hello Rustaceans! Welcome to a Rust Fullstack Application running on Docker and Kubernetes
				</p>
				<p>This is a demo app using:
					<ul>
						<li>React for front end code</li>
						<li>Bootstrap for styling because it was easy to add in</li>
						<li>Actix for speedy type-safe web serving</li>
						<li>SQLx for speedy type-safe MySQL connections</li>
						<li>MeiliSearch for searching because its an awesome Rust-based search engine</li>
						<li>WebAssembly to do some type-safe reliable number crunching</li>
					</ul>
				</p>
				<p>
					<Button
						variant="primary"
						onClick={handleCarsClick}>
						{
							isShowingCars ?
								'Hide the cars' :
								'Show me some cars'
						}
					</Button>
				</p>
			</StyledJumbotron>
			{
				isShowingCars ?
					<Cars /> :
					null
			}
		</Container>
		<Container as="footer">
			<p>
				<small>
					See also the <a href="https://github.com/camsjams/rust-fullstack-docker-k8s">GitHub repo</a>
				</small>
			</p>
		</Container>
	</Fragment>;
};

export default App;
