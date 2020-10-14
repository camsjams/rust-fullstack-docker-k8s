import React, {FC} from 'react';
import {Navbar, Nav} from 'react-bootstrap';
import Search from './Search';

const MainNav: FC = () =>
	<Navbar bg="light"
		expand="lg">
		<Navbar.Brand href="/">Rust and React Fullstack Demo</Navbar.Brand>
		<Navbar.Toggle aria-controls="basic-navbar-nav" />
		<Navbar.Collapse id="basic-navbar-nav">
			<Nav className="mr-auto">
				<Nav.Link href="/">Home</Nav.Link>
			</Nav>
			<Search searchHost="/indexes/cars/search" />
		</Navbar.Collapse>
	</Navbar>;

export default MainNav;
