export interface SafeWorker<In> extends Omit<Worker, "postMessage"> {
    postMessage(instruction: In): void;
}

export type ComputeInstruction =
| { type: "test"; width: number; height: number; row0: number; rows: number };

export type ComputeMessage =
| { type: "image"; width: number; height: number; row0: number; rows: number; data: ImageData };