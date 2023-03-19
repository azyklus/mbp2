# GraphQL API service for the MBP2 program.

import
   stew/results,
   std/logging

from ./config import nil

when defined(release):
   {.push checks: off, inline.}

#-------#

proc Main() =
   # Logging handlers
   var logger = newConsoleLogger()
   var fileLog = newFileLogger("errors.log", levelThreshold=lvlError)
   var rollingLog = newRollingFileLogger("rolling.log")
   addHandler(logger)
   addHandler(fileLog)
   addHandler(rollingLog)

   info "> Hello, World!"
   info "> Database program for blog/portfolio app, version 2."

   let cfg = tryGet config.Setup()

   info "> Database bound to ", cfg.BindAddr

#-------#

when isMainModule:
   Main()

when defined(release):
   {.pop.}
