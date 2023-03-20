# GraphQL API service for the MBP2 program.

import
   std/logging,
   stew/results,
   morelogging

from ./config import nil

when defined(release):
   {.push checks: off, inline.}

#-------#

proc Main() =
   # Logging handlers
   addHandler newStdoutLogger()
   addHandler newAsyncRotatingFileLogger(levelThreshold=lvlAll)
   addHandler newAsyncFileLogger("errors.log", levelThreshold=lvlError)

   info "> Hello, World!"
   info "> Database program for blog/portfolio app, version 2."

   let cfg = tryGet config.Setup()

   info "> Database bound to ", cfg.BindAddr

#-------#

when isMainModule: Main()

when defined(release):
   {.pop.}
