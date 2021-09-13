export type ComputeInstruction =
| { type: "test"; width: number; height: number };

export type ComputeMessage =
| { type: "ready" }
| { type: "image"; data: ImageData };