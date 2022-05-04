

ben@Algo-Guidarelli:~/go/src/github.com/barnjamin/indexoor$ cat indexer.go
package main

import (
        "context"
        "strings"

        "github.com/algorand/go-algorand/rpcs"
        "github.com/algorand/indexer/fetcher"
        "github.com/sirupsen/logrus"
)

var log = logrus.New()

func main() {
        f, err := fetcher.ForNetAndToken("http://localhost:4001", strings.Repeat("a", 64), log)
        if err != nil {
                log.Fatalf("Failed to create fetcher: %+v", err)
        }

        f.SetBlockHandler(handler)

        f.Run(context.Background())
}

func handler(ctx context.Context, cert *rpcs.EncodedBlockCert) error {
        for _, stxn := range cert.Block.Payset {
                log.Printf("%+v", stxn.SignedTxn.Txn.Type)
        }
        return nil
}

--

current algorand machine size:

  https://howbigisalgorand.com/

custom indexes:

  https://github.com/algorand/indexer/blob/develop/docs/PostgresqlIndexes.md

Installing node:

  https://developer.algorand.org/docs/run-a-node/setup/install/


kubectl exec -it algorand-0 -c algorand-algod -- /bin/bash

docker exec -it algorand-tilt-indexer /bin/bash

to switch to sandbox, change devnet/node.yaml

-            - http://algorand:8980
+            - http://host.minikube.internal:8980

put into dev/node.yaml

            - --algorandAppID
            - "4"

Install the algorand requirements

  python3 -m pip  install  -r requirements.txt 

install docker-compile

./sandbox down; ./sandbox clean; ./sandbox up dev -v; python3 admin.py --devnet

