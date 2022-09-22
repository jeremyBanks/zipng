import { sortBy } from "https://deno.land/std@0.156.0/collections/mod.ts";
import { IS_BROWSER } from "https://deno.land/x/fresh@1.1.1/runtime.ts";

export default ({ chapter }: { chapter: { html: string } }) => (
  <nav class="flex">
    <button
      disabled={!IS_BROWSER}
      onClick={(event) => {
        if (globalThis.speechSynthesis.speaking) {
          globalThis.speechSynthesis.cancel();
          return;
        }
        const voices = sortBy(globalThis.speechSynthesis.getVoices(), (v) =>
          (/natural/i.test(v.name) ? 0 : 1 << 9) +
          (/^en\-(CA)/i.test(v.lang) ? 0 : 1 << 8) +
          (/^richard/i.test(v.lang) ? 0 : 1 << 7) +
          (v.localService ? 0 : 1 << 6) +
          (/^en/i.test(v.lang) ? 0 : 1 << 5) +
          (v.default ? 0 : 1 << 4) +
          (/^en\-(US)/i.test(v.lang) ? 0 : 1 << 3) +
          (/^en\-(IN)/i.test(v.lang) ? 0 : 1 << 2));
        globalThis.speechSynthesis.cancel();
        const utterance = new globalThis.SpeechSynthesisUtterance(
          (new DOMParser()).parseFromString(chapter.html, "text/html").body
            .textContent!,
        );
        utterance.lang = "en-US";
        utterance.voice = voices[0];
        console.log(utterance);
        globalThis.speechSynthesis.speak(utterance);
      }}
      class="bg-blue-500 flex-grow p-4 text-white font-bold"
    >
      Read
    </button>
  </nav>
);
