const fs = require("fs")
const glob = require("glob")
const path = require("path")
const purify = require("purify-css")
const sass = require("sass")
const util = require("util")

const main = async () => {
	const result = await util.promisify(sass.render)({
		file: path.join(__dirname, "style", "index.sass"),
	})

	const hb = await util.promisify(glob)(path.join(__dirname, "..", "templates", "**/*.hbs"))
	const js = [path.join(__dirname, "..", "build", "main.js")]

	const mat = await util.promisify(fs.readFile)(path.join(__dirname, "style", "materialize.css"), "utf8")
	const matIcons = await util.promisify(fs.readFile)(path.join(__dirname, "style", "materialize-icons.css"), "utf8")

	await new Promise(resolve => purify(hb.concat(js), result.css.toString() + mat + matIcons, {
		minify: true,
		info: true,
		output: path.join(__dirname, "..", "build", "style.css"),
	}, resolve))
}

main().catch(console.error)
