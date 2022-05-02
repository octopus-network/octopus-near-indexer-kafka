# Octopus NEAR Indexer Kafka

Octopus NEAR Indexer Kafka is built on top of [NEAR Indexer microframework](https://github.com/nearprotocol/nearcore/tree/master/chain/indexer) to watch the network and store all the events in the Kafka.


## Roadmap

#### NEAR Lake

- [ ] migrate to NEAR Lake S3 (As a plug -in form [octopus-near-indexer-s3](https://github.com/octopus-network/octopus-near-indexer-s3))

### Prepare

Clone this repository and open the project folder

```bash
$ git clone git@github.com:octopus-network/octopus-near-indexer-kafka.git
$ cd octopus-near-indexer-kafka
```

### Compile Octopus NEAR Indexer Kafka

```bash
$ cargo build --release
```


### Configure Octopus NEAR Indexer Kafka

To connect Octopus NEAR Indexer Kafka to the specific chain you need to have necessary configs, you can generate it as follows:

```bash
$ ./target/release/generate --home ~/.near/testnet init --chain-id testnet --download-config --download-genesis
```

The above code will download the official genesis config and generate necessary configs. You can replace `testnet` in the command above to different network ID (`betanet`, `mainnet`).

**NB!** According to changes in `nearcore` config generation we don't fill all the necessary fields in the config file.
While this issue is open https://github.com/nearprotocol/nearcore/issues/3156 you need to download config you want and replace the generated one manually.
- [testnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json)
- [betanet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/betanet/config.json)
- [mainnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json)

Configs for the specified network are in the `--home` provided folder. We need to ensure that Octopus NEAR Indexer Kafka follows
all the necessary shards, so `"tracked_shards"` parameters in `~/.near/testnet/config.json` needs to be configured properly.
Currently, `nearcore` treats empty value for `"tracked_shards"` as "do not track any shard" and **any value** as "track all shards".
For example, in order to track all shards, you just add the shard #0 to the list:

```
...
"tracked_shards": [0],
...
```

### Run Octopus NEAR Indexer Kafka

Commands to run Octopus NEAR Indexer Kafka, after `./target/release/indexer`

| Command 	     | Key/Subcommand               	 | Required/Default                                                 	 | Responsible for                                                                                                                                                                                                                                                                                                                                                         	 |
|---------------|--------------------------------|--------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 	             | `--home`                 	     | Default <br>`~/.near`                                            	 | Tells the node where too look for necessary files: <br>`config.json`, <br>`genesis.json`, <br>`node_key.json`,<br> and <br>`data`<br> folder                                                                                                                                                                                                                    	         |
| `generate`  	 | 	                              | 	                                                                  | Tells the node to generate config files in `--home`                                                                                                                                                                                                                                                                                                                 	     |
| 	             | `--chain-id`                 	 | Required<br><br>  * `localnet`<br>  * `testnet`<br>  * `mainnet` 	 | Defines the chain to generate config files for                                                                                                                                                                                                                                                                                                                          	 |
| 	             | `--download-config`          	 | Optional                                                         	 | If provided tells the node to download `config.json` from the public URL. You can download them manually<br><br> - [testnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json)<br> - [mainnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json)      	 |
| 	             | `--download-genesis`         	 | Optional                                                         	 | If provided tells the node to download `genesis.json` from the public URL. You can download them manually<br><br> - [testnet genesis.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/genesis.json)<br> - [mainnet genesis.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/genesis.json) 	 |
| 	             | TODO:<br>Other `neard` keys  	 | 	                                                                  | 	                                                                                                                                                                                                                                                                                                                                                                         |
| `run`   	     | 	                              | 	                                                                  | Runs the node                                                                                                                                                                                                                                                                                                                                                           	 |
| 	             | `--stream-while-syncing`     	 | Optional                                                         	 | If provided Indexer streams blocks while they appear on the node instead of waiting the node to be fully synced                                                                                                                                                                                                                                                         	 |
| 	             | `--concurrency`              	 | Default 1                                                        	 | Defines the concurrency for the process of saving block data to Kafka                                                                                                                                                                                                                                                                                                  	  |
| 	             | `sync-from-latest`           	 | One of the `sync-` subcommands is required                       	 | Tells the node to start indexing from the latest block in the network                                                                                                                                                                                                                                                                                                   	 |
| 	             | `sync-from-interruption`     	 | One of the `sync-` subcommands is required                       	 | Tells the node to start indexing from the block the node was interrupted on (if it is a first start it will fallback to `sync-from-latest`)                                                                                                                                                                                                                             	 |
| 	             | `sync-from-block --height N` 	 | One of the <br>`sync-`<br> subcommands is required               	 | Tells the node to start indexing from the specified block height `N` (**Ensure** you node data has the block you want to start from)                                                                                                                                                                                                                                    	 |

```bash
$ ./target/release/idexer --home ~/.near/testnet run --stream-while-syncing --concurrency 50 sync-from-latest
```

After the network is synced, you should see logs of every block height currently received by Octopus NEAR Indexer Kafka.

