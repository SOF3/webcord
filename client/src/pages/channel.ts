export interface PageConfig {
}

export function main(config: PageConfig) {
	const elems = document.querySelectorAll(".dropdown-trigger")
	M.Dropdown.init(elems, {
		hover: true,
	})
}
