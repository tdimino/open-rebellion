
void __fastcall FUN_005d58c0(int param_1)

{
  uint uVar1;
  int iVar2;
  
  if (*(int *)(param_1 + 0x68) == 0) {
    uVar1 = FUN_00401a00();
    if ((uVar1 & 0x2000000) != 0) {
      iVar2 = (**(code **)(**(int **)(param_1 + 0x2c) + 0x48))
                        (*(int **)(param_1 + 0x2c),*(undefined4 *)(param_1 + 0x3c),0);
      FUN_005d8b00(iVar2);
      iVar2 = (**(code **)(**(int **)(param_1 + 0x2c) + 0x48))
                        (*(int **)(param_1 + 0x2c),*(undefined4 *)(param_1 + 0x40),0);
      FUN_005d8b00(iVar2);
      *(undefined4 *)(param_1 + 0x68) = 1;
    }
  }
  return;
}

