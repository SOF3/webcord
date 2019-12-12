const path = require("path")

module.exports = {
	mode: "production",
	entry: ["./js/index.js", "./materialize.js"],
	output: {
		path: path.join(__dirname, "..", "build"),
	},
}
