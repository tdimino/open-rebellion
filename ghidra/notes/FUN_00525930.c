
int __fastcall FUN_00525930(int param_1)

{
  int iVar1;
  
  *(undefined4 *)(param_1 + 0x1c) = 0;
  *(undefined4 *)(param_1 + 0x20) = 0;
  *(undefined4 *)(param_1 + 0x24) = 0;
  FUN_004ece80((undefined4 *)(param_1 + 0x28));
  *(undefined4 *)(param_1 + 0x2c) = 0;
  if (*(int *)(param_1 + 0x10) == 0) {
    if (*(int *)(param_1 + 0x14) == 0) {
      if (*(int *)(param_1 + 0x18) != 0) {
        *(undefined4 *)(param_1 + 0x24) = 1;
      }
    }
    else {
      *(undefined4 *)(param_1 + 0x20) = 1;
    }
  }
  else {
    *(undefined4 *)(param_1 + 0x1c) = 1;
  }
  FUN_00525a50(param_1);
  FUN_005259a0(param_1);
  iVar1 = FUN_00525a20(param_1);
  if (iVar1 == 0) {
    FUN_005258f0(param_1);
  }
  return param_1;
}

