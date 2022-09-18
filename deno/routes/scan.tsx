import { PageProps } from "$fresh/server.ts";
import Page from "../components/Page.tsx";
import BarcodeScanner from "../islands/Scanner.tsx";

export default function IndexPage(props: PageProps) {
  return (
    <Page>
      <BarcodeScanner />
    </Page>
  );
}
