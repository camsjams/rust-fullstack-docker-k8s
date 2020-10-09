import React, {PureComponent, ReactNode, ChangeEvent, Fragment} from 'react';
import {Form, FormControl, Modal, Button} from 'react-bootstrap';
import debounce from 'lodash.debounce';
import CarsList from './CarsList';

type Props = {
	searchHost: string;
};

type State = {
	isOpen: boolean;
	isLoading: boolean;
	isError: boolean;
	results: Cars;
	value: string;
};

class Search extends PureComponent<Props, State> {
	constructor(props: Props) {
		super(props);

		this.state = {
			isOpen: false,
			isLoading: false,
			isError: false,
			results: [],
			value: ''
		};

		this.handleCtaClick = this.handleCtaClick.bind(this);
		this.handleClose = this.handleClose.bind(this);

		this.handleSearchChange = this.handleSearchChange.bind(this);
		this.runSearch = debounce(this.runSearch.bind(this), 500, {leading: true});
	}

	handleCtaClick(): void {
		this.setState({
			isOpen: true
		});
	}

	handleClose(): void {
		this.setState({
			isOpen: false
		});
	}

	async handleSearchChange(event: ChangeEvent<HTMLInputElement>): Promise<void> {
		const value = event.target.value;
		this.setState({
			isLoading: true,
			value: value || ''
		});
		if (!value || value.length < 2) {
			return this.setState({
				isLoading: false,
				results: []
			});
		}

		return this.runSearch(value)
			.catch((error) => {
				console.error(error);

				this.setState({
					isLoading: false,
					isError: true
				});
			});
	}

	async runSearch(value: string): Promise<void> {
		const response = await fetch(`${this.props.searchHost}/search?q=${value}&attributesToHighlight=*`);
		if (response.ok) {
			const json = await response.json();
			this.setState({
				results: json.hits as Cars,
				isLoading: false
			});
		} else {
			this.setState({
				isLoading: false,
				isError: true
			});
		}
	}

	render(): ReactNode {
		const {isOpen, isError, value, results} = this.state;
		return <Fragment>
			<Form inline>
				<FormControl
					onClick={this.handleCtaClick}
					type="text"
					placeholder="Search for cars"
					className="mr-sm-2" />
			</Form>
			<Modal
				show={isOpen}
				size="xl"
				onHide={this.handleClose}>
				<Modal.Header closeButton>
					<Modal.Title>Car Search: Powered By MeiliSearch</Modal.Title>
				</Modal.Header>
				<Modal.Body>
					<Form inline>
						<FormControl
							onChange={this.handleSearchChange}
							type="text"
							value={value}
							placeholder="Search for cars"
							className="mr-sm-2" />
					</Form>
					{
						isError ? 'Something went wrong' : null
					}
					<CarsList cars={results} />
				</Modal.Body>
				<Modal.Footer>
					<Button variant="secondary"
						onClick={this.handleClose}>
						Close
					</Button>
				</Modal.Footer>
			</Modal>
		</Fragment>;
	}
}

export default Search;
