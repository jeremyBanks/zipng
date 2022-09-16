import { IS_BROWSER } from "$fresh/runtime.ts";
import { useSignal } from "@preact/signals";
import { useState } from "preact/hooks";
import BarcodeScanner from "../components/Scanner.tsx";

export default function () {
  return <BarcodeScanner />;
}
