
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __fastcall FUN_005d9640(int param_1)

{
  int iVar1;
  
  if (0xb4 < *(int *)(param_1 + 0x3c)) {
    *(int *)(param_1 + 0x3c) = *(int *)(param_1 + 0x3c) + -0x168;
  }
  if (*(int *)(param_1 + 0x3c) < -0xb4) {
    *(int *)(param_1 + 0x3c) = *(int *)(param_1 + 0x3c) + 0x168;
  }
  iVar1 = (**(code **)(**(int **)(param_1 + 8) + 0x44))
                    (*(int **)(param_1 + 8),0,0,0x3f800000,0,
                     (float)*(int *)(param_1 + 0x3c) * (float)_DAT_006bcc00);
  *(int *)(param_1 + 0x2c) = iVar1;
  if (iVar1 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar1);
  }
  iVar1 = (**(code **)(**(int **)(param_1 + 8) + 0x44))
                    (*(int **)(param_1 + 8),1,0x3f800000,0,0,
                     (float)*(int *)(param_1 + 0x38) * (float)_DAT_006bcc00);
  *(int *)(param_1 + 0x2c) = iVar1;
  if (iVar1 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar1);
  }
  iVar1 = (**(code **)(**(int **)(param_1 + 8) + 0x3c))
                    (*(int **)(param_1 + 8),1,0,0,-*(float *)(param_1 + 0x4c));
  *(int *)(param_1 + 0x2c) = iVar1;
  if (iVar1 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar1);
  }
  iVar1 = (**(code **)(**(int **)(param_1 + 4) + 0x40))
                    (*(int **)(param_1 + 4),*(undefined4 *)(param_1 + 0x48));
  *(int *)(param_1 + 0x2c) = iVar1;
  if (iVar1 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar1);
  }
  if ((*(uint *)(param_1 + 0x44) != 0) && (*(int *)(param_1 + 0x40) == 0)) {
    iVar1 = FUN_005c1080(*(uint *)(param_1 + 0x44));
    if (iVar1 != 0) {
      *(undefined4 *)(param_1 + 0x40) = *(undefined4 *)(iVar1 + 0x2c);
    }
  }
  if (*(int *)(param_1 + 0x40) != 0) {
    iVar1 = (**(code **)(**(int **)(param_1 + 8) + 0x90))
                      (*(int **)(param_1 + 8),*(int *)(param_1 + 0x40),
                       *(undefined4 *)(param_1 + 0xc),0);
    *(int *)(param_1 + 0x2c) = iVar1;
    if (iVar1 != 0) {
      FUN_005d93f0(*(int *)(param_1 + 0x10),iVar1);
    }
  }
  iVar1 = (**(code **)(**(int **)(param_1 + 8) + 0x7c))
                    (*(int **)(param_1 + 8),*(undefined4 *)(param_1 + 0xc),param_1 + 0x14,
                     param_1 + 0x20);
  *(int *)(param_1 + 0x2c) = iVar1;
  if (iVar1 != 0) {
    FUN_005d93f0(*(int *)(param_1 + 0x10),iVar1);
  }
  (**(code **)(**(int **)(param_1 + 4) + 0x38))(*(int **)(param_1 + 4),0x3f800000);
  return 1;
}

