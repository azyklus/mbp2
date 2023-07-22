# GraphQL API service for the MBP2 program.

import
   mongo,
   std/[
      logging, uri
   ],
   stew/results

from ./config import nil

when defined(release):
   {.push checks: off, inline.}

#-------#

proc Main() =
   # Logging handlers
   addHandler newConsoleLogger()
   addHandler newFileLogger("all.log", levelThreshold=lvlAll)
   addHandler newRollingFileLogger("errors.log", levelThreshold=lvlError)

   info "> Hello, World!"
   info "> Database program for blog/portfolio app, version 2."

   let cfg = tryGet config.Setup()

   info "> Database bound to ", cfg.BindAddr

   var db = newMongo(parseUri cfg.BindAddr)

#-------#

when isMainModule: Main()

when defined(release):
   {.pop.}
