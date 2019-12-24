const fs = require("fs")
const glob = require("glob")
const path = require("path")
const sass = require("sass")
const uglify = require("uglifycss")
const util = require("util")

const main = async () => {
	const result = await util.promisify(sass.render)({
		file: path.join(__dirname, "style", "index.sass"),
	})

	const mat = await util.promisify(fs.readFile)(path.join(__dirname, "style", "materialize.css"), "utf8")
	const matIcons = await util.promisify(fs.readFile)(path.join(__dirname, "style", "materialize-icons.css"), "utf8")

	const outputCss = uglify.processString(result.css.toString() + mat + matIcons, {
		uglyComments: true,
	})
	await util.promisify(fs.writeFile)(path.join(__dirname, "..", "build", "style.css"), outputCss)
}

main().catch(console.error)
