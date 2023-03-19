# Package

packageName       = "mbp2_db"
namedBin["index"] = "mbp2_database"

version           = "0.1.0"
author            = "Yarot Kell"
description       = "GraphQL API service for MBP2."
license           = "Apache-2.0"
srcDir            = "."


# Dependencies

requires "nim >= 1.6.10",
         "dotenv",
         "chronos",
         "stew",
         "unittest2",
         "https://github.com/treeform/jsony",
         "https://github.com/azyklus/mongo",
         "https://github.com/status-im/nim-graphql"
