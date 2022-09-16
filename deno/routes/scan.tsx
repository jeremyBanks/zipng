import { PageProps } from "https://deno.land/x/fresh/server.ts";
import BarcodeScanner from "../islands/Scanner.tsx";

export default function IndexPage(props: PageProps) {
  return (
    <main>
      <BarcodeScanner />
    </main>
  );
}
