const dataUrlBase = new URL("https://sfic.s3.amazonaws.com/");
const dataFileBase = "../data/";
const extraFileBase = "../target/";

const cache = new Array<[string, string]>();
const cacheItemCapacity = 128;
const cacheCharacterCapacity = 4 * 1024 * 1024;

export const load = async (path: string) => {
  const errors: unknown[] = [];

  const text = cache.find(([k, _v]) => k === path)?.[1] ??
    await Deno.readTextFile(dataFileBase + path).catch((error) => {
      errors.push(error);
      return Deno.readTextFile(extraFileBase + path);
    }).catch(async (error) => {
      errors.push(error);
      const url = new URL(path, dataUrlBase);
      const response = await fetch(url);
      if (!response.ok) {
        throw new Error(`HTTP Error ${response.status} ${response.statusText}`);
      }
      return response.text();
    }).catch((error) => {
      errors.push(error);
      throw new Error(`Failed to load ${path}:\n${errors.join("\n")}`);
    });

  if (text.length > cacheCharacterCapacity) {
    console.warn(
      `A single response value is larger (${text.length} chars) than our total response cache capacity (${cacheCharacterCapacity} characters)? That's iffy.`,
    );
  } else if (cache.at(-1)?.[0] !== path) {
    cache.push([path, text]);
  }

  while (
    cache.length > cacheItemCapacity ||
    [...new Set(cache.flat())].reduce((a, b) => a + b.length, 0) >
      cacheCharacterCapacity
  ) {
    cache.shift();
  }

  return JSON.parse(text);
};
