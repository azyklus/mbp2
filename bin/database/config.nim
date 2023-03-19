import
   std/[
      jsonutils,
      marshal,
      os, parseopt,
      strformat,
      strutils,
   ],
   chronos

from dotenv import nil
from std/json import nil
from stew/results import nil

type
   Schema* = enum
      Blog
      Repository

   VersionCheck* = object
      Major*:    uint8
      Minor*:    uint8
      Revision*: uint8
      Bump*:     uint8

   Configuration* = ref object
      BindAddr*: string
      Schema*: Schema
      Secure*: bool

var testConfig {.threadvar.}: Configuration

const defaultAddr = "127.0.0.1:8547"

proc Setup*(): results.Result[Configuration, string] =
   var res = new Configuration
   var configFile: File
   var configData: string
   var configJson: json.JsonNode

   if open(configFile, "AppSettings.json"):
      try:
         configData = readAll(configFile)
         configJson = json.parseJson(configData)

         res = json.to(configJson, Configuration)
         result = results.ok(res)
      except IOError:
         let e = getCurrentException()
         echo "Got exception ", repr(e), " with message: ", e.msg
         echo "Error was fatal and this program cannot continue."

         result = results.err(fmt "{e.msg}")
      finally:
         close(configFile)
   else:
      let e = newException(IOError, "could not open file")

      res.BindAddr = defaultAddr
      res.Schema = Schema.Blog
      configJson = toJson(res)
      configData = json.pretty(configJson)

      writeFile("AppSettings.json", configData)

      return results.err(fmt "{e.msg}")

   return result
