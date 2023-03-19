import
   std/[
      jsonutils,
      logging,
      marshal,
      os, parseopt,
      strformat,
      strutils,
      unicode,
   ],
   chronos

from dotenv import nil
from jsony import nil
from std/json import nil
from stew/results import nil

type
   Schema* = enum
      Blog
      Repository
      Unknown

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

proc enumHook*(s: string): Schema =
   case s:
   of "Blog":
      return Schema.Blog
   of "Repository":
      return Schema.Repository
   else:
      return Schema.Unknown

proc renameHook*(node: var json.JsonNode, fieldName: var string) =
   if isLowerAscii fieldName[0]:
      fieldName[0] = toUpperAscii fieldName[0]
   elif fieldName == "type":
      fieldName = "Kind"

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
         error "> Got exception ", repr(e), " with message: ", e.msg
         error "> Error was fatal and this program cannot continue."

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

   dotenv.load("../..")

   return result
