let configIn = ""

function ReadConfig(file) {
   if (file.type && file.type.startsWith("text/")) {
      console.log("config file not loaded", file.type, file);
      return;
   }

   const reader = new FileReader();
   reader.addEventListener('load', (event) => {
      configIn = event.target.result
   });
   reader.readAsText(file, "utf8");
   return configIn;
}
