import dev from "$fresh/dev.ts";
import "preact/devtools";

await dev(import.meta.url, "./main.ts");
