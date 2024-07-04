import RevealNotes from "./reveal/plugin/notes/notes.esm.js";
import Reveal from "./reveal/reveal.esm.js";

const deck = new Reveal({
  plugins: [RevealNotes],
});

deck.initialize();
