# GraphQL API service for the MBP2 program.

import stew/results
from ./config import nil

when defined(release):
   {.push checks: off, inline.}

#-------#

proc Main() =
   echo "Hello, World!"
   echo "Database program for blog/portfolio app, version 2."

   let cfg = tryGet config.Setup()

   echo "Database bound to ", cfg.BindAddr

#-------#

when isMainModule:
   Main()

when defined(release):
   {.pop.}
