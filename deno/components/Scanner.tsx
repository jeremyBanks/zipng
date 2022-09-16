import { IS_BROWSER } from "$fresh/runtime.ts";
import { useSignal } from "@preact/signals";
import { useState } from "preact/hooks";

export default function BarcodeScanner() {
  const enabled = IS_BROWSER;
  const activated = useSignal(false);
  const [clickCount, setClickCount] = useState(0);

  if (!activated.value) {
    return (
      <div>
        {clickCount}
        <button
          disabled={!enabled}
          class="p-2 m-8 bg-red-800 text-white font-bold hover:bg-red-600"
          onClick={() => {
            activated.value = true;
            setClickCount(clickCount + 1);
          }}
        >
          Start Scanning
        </button>
      </div>
    );
  } else {
    return (
      <div>
        {clickCount}
        <button
          disabled={!enabled}
          class="p-2 m-8 bg-green-800 text-white font-bold hover:bg-green-600"
          onClick={() => {
            activated.value = false;
            setClickCount(clickCount + 1);
          }}
        >
          Stop Scanning
        </button>
      </div>
    );
  }
}
