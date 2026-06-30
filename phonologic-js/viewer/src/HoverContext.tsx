import {createContext} from "react";

type HoverContextState = [number|undefined, (x: number|undefined) => void];
export const HoverContext = createContext<HoverContextState>([undefined, (x) => {}]);