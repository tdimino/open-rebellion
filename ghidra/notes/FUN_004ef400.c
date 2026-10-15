
void __thiscall FUN_004ef400(int param_1,int param_2,undefined4 param_3,undefined4 param_4)

{
  if (*(short *)(param_1 + 0x96) == param_2) {
    FUN_004ee350(0,param_4);
    return;
  }
  FUN_004ee350(param_2,param_4);
  if (*(short *)(param_1 + 0x96) != 0) {
    FUN_004ee740(param_3,param_4);
  }
  return;
}

