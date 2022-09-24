import { sortBy } from "https://deno.land/std@0.156.0/collections/sort_by.ts";
import { IS_BROWSER } from "https://deno.land/x/fresh@1.1.1/runtime.ts";
import { apply, css, tw } from "twind/css";
import { effect, useComputed, useSignal } from "@preact/signals";
import { useEffect, useMemo } from "preact/hooks";

const { DOMParser } = IS_BROWSER
  ? globalThis
  : await import("deno-dom") as unknown as typeof globalThis;

export default ({ chapter }: { chapter: { title: string; html: string } }) => {
  const voices = useSignal([] as SpeechSynthesisVoice[]);
  const rankedVoices = useComputed(() =>
    voices.value &&
      sortBy(
        voices.value.map((v) => ({
          r: Math.random(),
          voice: v,
        })),
        ({ voice, r }) => {
          let n = 2;
          n = n * 2 + (/^en/i.test(voice.lang) ? 0 : 1);
          n = n * 2 + (/(Natural)/i.test(voice.name) ? 0 : 1);
          n = n * 2 + (/^en\-(CA)/i.test(voice.lang) ? 0 : 1);
          n = n * 2 + (/^en\-(US)/i.test(voice.lang) ? 0 : 1);
          n = n * 2 + (voice.default ? 0 : 1);
          n = n * 2 + (voice.localService ? 0 : 1);
          n = n + r;
          return n;
        },
      )
        .map(({ voice }) => voice) || []
  );

  const chapterChunks = useMemo(() => {
    const chapterBody = (new DOMParser()).parseFromString(
      chapter.html.replace(/<br\s*\/?>\s*<br\s*\/?>/ig, "<p>"),
      "text/html",
    ).body;

    let chapterRoot: Element = chapterBody;
    while (
      chapterRoot.children.length == 1 &&
      chapterRoot.children[0].textContent == chapterRoot.textContent
    ) {
      chapterRoot = chapterRoot.children[0];
    }

    return [
      {
        text: chapter.title,
        type: "heading",
        breaks: true,
      },
      ...Array.from(chapterRoot.childNodes).flatMap((n) => {
        const text = n.textContent?.trim() ?? "";
        const chunks: {
          text: string;
          type: "narration" | "dialog" | "heading";
          breaks?: boolean;
        }[] = [];
        const [narration, ...dialogue] = text.split(/("|«|‛|“|‟|‹|\[|^')/g);
        chunks.push({
          text: narration,
          type: "narration",
        });
        for (const d of dialogue) {
          const [speaker, ...rest] = d.split(/("|»|’|”|”|›|\]|'$)/g);
          chunks.push({
            text: speaker,
            type: "dialog",
          });
          for (const text of rest) {
            chunks.push({
              text,
              type: "narration",
            });
          }
        }
        const filtered = chunks.filter((c) => c.text?.trim());
        if (filtered.length) {
          filtered.at(-1)!.breaks = true;
        }
        return filtered;
      }),
    ];
  }, [chapter.html]);

  useEffect(() => {
    const onvoiceschanged = () => {
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
      <button
        class={tw`${
          !IS_BROWSER && `invisible`
        } bg-blue-500 flex-grow p-4 text-white font-bold`}
        onClick={async (click) => {
          if (speechSynthesis.paused) {
            speechSynthesis.resume();
          }
          if (speechSynthesis.speaking) {
            speechSynthesis.cancel();
            return;
          }
          for (const chunk of chapterChunks) {
            let delay = chunk.breaks ? 250 : 0;
            const utterance = new SpeechSynthesisUtterance(chunk.text);
            const uttered = new Promise((resolve, reject) => {
              utterance.onend = resolve;
              utterance.onerror = reject;
            });
            utterance.lang = "en-US";
            if (chunk.type == "narration") {
              utterance.voice = rankedVoices.value[0];
            } else if (chunk.type == "heading") {
              utterance.voice = rankedVoices.value[0];
              utterance.rate = 6 / 8;
              utterance.volume = 9 / 8;
              delay = 500;
            } else if (chunk.type == "dialog") {
              utterance.voice = rankedVoices.value[1];
              utterance.rate = 9 / 8;
            }
            speechSynthesis.speak(utterance);
            await uttered;
            if (delay) {
              await new Promise((resolve) => setTimeout(resolve, delay));
            }
          }
        }}
      >
        Read
      </button>
    </nav>
  );
};
