
int __fastcall FUN_004c8740(int param_1)

{
  int *piVar1;
  int iVar2;
  
  iVar2 = 0;
  piVar1 = (int *)FUN_005f5500((void *)(*(int *)(param_1 + 0x2c) + 0xec),*(uint *)(param_1 + 0x4c));
  if (piVar1 == (int *)0x0) {
    piVar1 = (int *)thunk_FUN_005f5060(*(int *)(param_1 + 0x2c) + 0xec);
    if (piVar1 != (int *)0x0) goto LAB_004c8773;
  }
  else {
LAB_004c8773:
    do {
      if (iVar2 != 0) break;
      iVar2 = (**(code **)(*piVar1 + 0x10))();
      if (iVar2 == 0) {
        piVar1 = (int *)piVar1[4];
      }
    } while (piVar1 != (int *)0x0);
    if (piVar1 != (int *)0x0) {
      *(int *)(param_1 + 0x48) = piVar1[6];
      if (piVar1[4] != 0) {
        *(undefined4 *)(param_1 + 0x4c) = *(undefined4 *)(piVar1[4] + 0x18);
        return iVar2;
      }
      goto LAB_004c87af;
    }
  }
  *(undefined4 *)(param_1 + 0x48) = 0;
LAB_004c87af:
  *(undefined4 *)(param_1 + 0x4c) = 0;
  return iVar2;
}

