
void __thiscall FUN_004ef070(int param_1,undefined4 param_2)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = FUN_004ed220();
  if (iVar1 != 0) {
    iVar1 = FUN_0053eb70((int)*(short *)(param_1 + 0x8c));
    if ((((iVar1 != 0) && ((*(byte *)(param_1 + 0x78) & 0x80) == 0)) &&
        (uVar2 = 1, (*(byte *)(param_1 + 0x50) & 1) != 0)) && ((*(byte *)(param_1 + 0xac) & 1) == 0)
       ) goto LAB_004ef0ac;
  }
  uVar2 = 0;
LAB_004ef0ac:
  FUN_004eeaa0(uVar2,param_2);
  return;
}

