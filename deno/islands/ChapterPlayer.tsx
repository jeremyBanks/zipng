import { sortBy } from "https://deno.land/std@0.156.0/collections/mod.ts";
import { IS_BROWSER } from "https://deno.land/x/fresh@1.1.1/runtime.ts";
import { apply, css, tw } from "twind/css";
import { effect, useComputed, useSignal } from "@preact/signals";
import { useEffect, useMemo } from "preact/hooks";

const { DOMParser } = IS_BROWSER
  ? globalThis
  : await import("deno-dom") as unknown as typeof globalThis;

export default ({ chapter }: { chapter: { html: string } }) => {
  const voices = useSignal([] as SpeechSynthesisVoice[]);
  const preferredVoice = useComputed(() =>
    voices.value &&
      sortBy(
        voices.value,
        (v) => {
          let n = 3;
          n += n - +/^en/i.test(v.lang);
          n += n - +/(Natural)/.test(v.name);
          n += n - +/^en\-(CA)/.test(v.lang);
          n += n - +v.localService;
          n += n - +/Richard/.test(v.name);
          n += n - +/^en\-(US)/i.test(v.lang);
          n += n - +/^en\-(IN)/i.test(v.lang);
          n += n - +v.default;
          return n;
        },
      )[0] || null
  );

  const chapterText = useMemo(() => {
    const chapterBody =
      (new DOMParser()).parseFromString(chapter.html, "text/html").body;

    const chapterChunks = [];

    let chapterRoot = chapterBody;
    // while (chapterRoot.)

    // .textContent!,
  }, [chapter.html]);

  useEffect(() => {
    const onvoiceschanged = () => {
      console.log("Voices changed!");
      voices.value = globalThis.speechSynthesis?.getVoices();
    };

    globalThis.speechSynthesis?.addEventListener(
      "voiceschanged",
      onvoiceschanged,
    );

    voices.value = globalThis.speechSynthesis?.getVoices();

    return () =>
      void speechSynthesis?.removeEventListener(
        "voiceschanged",
        onvoiceschanged,
      );
  }, []);

  return (
    <nav class="flex">
      <p>
        Voice: {preferredVoice.value?.name} (of {voices.value?.length ?? 0}{" "}
        voices)
      </p>
      <button
        class={tw`${
          !IS_BROWSER && `invisible`
        } bg-blue-500 flex-grow p-4 text-white font-bold`}
        onClick={(click) => {
          if (speechSynthesis.paused) {
            speechSynthesis.resume();
          }
          if (speechSynthesis.speaking) {
            speechSynthesis.cancel();
            return;
          }
          const utterance = new SpeechSynthesisUtterance();
          utterance.lang = "en-US";
          utterance.voice = preferredVoice.value;
          console.log(utterance);
          speechSynthesis.speak(utterance);
        }}
      >
        Read
      </button>
    </nav>
  );
};
