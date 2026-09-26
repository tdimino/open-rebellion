
undefined4 __fastcall FUN_004dbd60(int param_1)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  int iVar4;
  void *pvVar5;
  undefined4 local_c;
  
  uVar2 = *(uint *)(param_1 + 0x40) >> 0x18;
  local_c = 0;
  *(undefined4 *)(param_1 + 0x4c) = 0;
  if ((uVar2 < 0x90) || (0x97 < uVar2)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    return 0;
  }
  iVar3 = FUN_004dd050(param_1);
  if (iVar3 < 1) {
    if (iVar3 < 0) {
      local_c = 5;
    }
  }
  else {
    iVar4 = FUN_0041b230(*(void **)(param_1 + 0x2c),(uint *)(param_1 + 0x48),(void *)0x2a,0x10000,
                         0x4000,2);
    if (((*(uint *)(param_1 + 0x48) & 0xff000000) != 0) && (iVar4 != 0)) {
      *(int *)(param_1 + 0x4c) = iVar3 / iVar4;
    }
    iVar3 = *(int *)(param_1 + 0x4c);
    if (iVar3 < 1) goto LAB_004dbe55;
    iVar3 = *(int *)(param_1 + 0x2c);
    FUN_004f26d0(&stack0xffffffe0,(undefined4 *)(param_1 + 0x40));
    pvVar5 = FUN_00403d30((void *)(iVar3 + 0x2c));
    if (pvVar5 != (void *)0x0) {
      iVar3 = *(int *)((int)pvVar5 + 0x84);
      if (*(int *)(param_1 + 0x50) < *(int *)((int)pvVar5 + 0x84)) {
        iVar3 = *(int *)(param_1 + 0x50);
      }
      if (iVar3 < *(int *)(param_1 + 0x4c)) {
        *(int *)(param_1 + 0x4c) = iVar3;
      }
      if (*(int *)(param_1 + 0x54) < *(int *)(param_1 + 0x4c)) {
        *(int *)(param_1 + 0x4c) = *(int *)(param_1 + 0x54);
      }
      local_c = 8;
    }
  }
  iVar3 = *(int *)(param_1 + 0x4c);
LAB_004dbe55:
  if (iVar3 == 0) {
    local_c = 6;
  }
  return local_c;
}

