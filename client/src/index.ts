import * as account from "./pages/account"
import * as channel from "./pages/channel"
import * as guilds from "./pages/guilds"
import * as index from "./pages/index"

export type WebcordClientConfig =
	["index", index.PageConfig] |
	["channel", channel.PageConfig] |
	["guilds", guilds.PageConfig] |
	["account", account.PageConfig]

declare const WEBCORD_CLIENT_CONFIG: WebcordClientConfig

const main = () => {
	const [page, config] = WEBCORD_CLIENT_CONFIG
	if(page === "account") {
		account.main(config)
	} else if(page === "channel") {
		channel.main(config)
	} else if(page === "guilds") {
		guilds.main(config)
	} else if(page == "index") {
		index.main(config)
	}
}

main()
