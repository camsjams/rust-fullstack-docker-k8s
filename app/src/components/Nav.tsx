import React, {FC} from 'react';
import {Navbar, Nav, Form, FormControl, Button} from 'react-bootstrap';

const MainNav: FC = () =>
	<Navbar bg="light"
		expand="lg">
		<Navbar.Brand href="/">Rust and React Fullstack Demo</Navbar.Brand>
		<Navbar.Toggle aria-controls="basic-navbar-nav" />
		<Navbar.Collapse id="basic-navbar-nav">
			<Nav className="mr-auto">
				<Nav.Link href="/">Home</Nav.Link>
			</Nav>
			<Form inline>
				<FormControl type="text"
					placeholder="Search"
					className="mr-sm-2" />
				<Button variant="outline-success">Search for cars</Button>
			</Form>
		</Navbar.Collapse>
	</Navbar>;

export default MainNav;
