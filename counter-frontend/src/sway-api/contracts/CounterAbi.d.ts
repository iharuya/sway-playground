/* Autogenerated file. Do not edit manually. */

/* tslint:disable */
/* eslint-disable */

/*
  Fuels version: 0.79.0
  Forc version: 0.49.3
  Fuel-Core version: 0.22.1
*/

import type {
  BigNumberish,
  BN,
  BytesLike,
  Contract,
  DecodedValue,
  FunctionFragment,
  Interface,
  InvokeFunction,
} from 'fuels';

interface CounterAbiInterface extends Interface {
  functions: {
    dec: FunctionFragment;
    get: FunctionFragment;
    inc: FunctionFragment;
  };

  encodeFunctionData(functionFragment: 'dec', values: []): Uint8Array;
  encodeFunctionData(functionFragment: 'get', values: []): Uint8Array;
  encodeFunctionData(functionFragment: 'inc', values: []): Uint8Array;

  decodeFunctionData(functionFragment: 'dec', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'get', data: BytesLike): DecodedValue;
  decodeFunctionData(functionFragment: 'inc', data: BytesLike): DecodedValue;
}

export class CounterAbi extends Contract {
  interface: CounterAbiInterface;
  functions: {
    dec: InvokeFunction<[], BN>;
    get: InvokeFunction<[], BN>;
    inc: InvokeFunction<[], BN>;
  };
}