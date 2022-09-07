/** @jsx h */
import { h } from "preact";
import { signal } from "@preact/signals";
import { tw } from "@twind";

import { Button } from "../components/Button.tsx";

const count = signal(3);

export default function Counter() {
  return (
    <div class={tw`flex gap-2 w-full`}>
      <p class={tw`flex-grow-1 font-bold text-xl`}>
        the count is {count.value}
      </p>
      <Button onClick={() => count.value -= 1}>-1</Button>
      <Button onClick={() => count.value += 1}>+1</Button>
    </div>
  );
}
