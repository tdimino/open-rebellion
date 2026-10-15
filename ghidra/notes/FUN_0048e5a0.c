
void __thiscall FUN_0048e5a0(int param_1,int param_2,undefined4 param_3,byte param_4)

{
  if ((param_2 == 0x128) && ((param_4 & 1) != 0)) {
    *(undefined4 *)(param_1 + 0x40) = 1;
    *(undefined4 *)(param_1 + 0x28) = 0x11;
    FUN_0060b9d0(param_1 + 0x44,0x7188,0,0,0,0);
    FUN_0060b9d0(param_1 + 0x50,0x7189,0,0,0,0);
    *(undefined4 *)(param_1 + 0x5c) = 0x73;
  }
  return;
}

