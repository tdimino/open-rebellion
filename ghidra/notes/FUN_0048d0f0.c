
void __thiscall FUN_0048d0f0(int param_1,int param_2,undefined4 param_3,int param_4)

{
  undefined2 uVar1;
  int iVar2;
  
  if (((param_2 == 0x210) && (param_4 != 0)) && (*(int *)(param_1 + 0x20) == 1)) {
    *(undefined4 *)(param_1 + 100) = 1;
    iVar2 = FUN_004f3e50(1);
    if (iVar2 != 0) {
      *(undefined2 *)(param_1 + 0x2e) = 0x412;
      *(ushort *)(param_1 + 0x30) = (*(int *)(param_1 + 0x20) != 1) + 0x47c;
      uVar1 = FUN_004c5000(iVar2);
      *(undefined2 *)(param_1 + 0x2c) = uVar1;
      *(undefined4 *)(param_1 + 0x28) = 0x16;
      FUN_0060b9d0(param_1 + 0x44,0x71ba,0,0,0,0);
      FUN_0060b9d0(param_1 + 0x50,0x71bb,0,0,0,0);
      *(undefined4 *)(param_1 + 0x40) = 1;
    }
  }
  return;
}

