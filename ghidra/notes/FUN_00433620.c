
undefined4 __fastcall FUN_00433620(void *param_1)

{
  undefined4 uVar1;
  int *piVar2;
  uint *puVar3;
  void *pvVar4;
  int iVar5;
  undefined4 *puVar6;
  void *local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d1f0;
  local_c = ExceptionList;
  uVar1 = 0;
  if (*(int *)((int)param_1 + 0x1c) != 0) {
    ExceptionList = &local_c;
    *(undefined4 *)((int)param_1 + 0x1c) = 0;
    *(uint *)((int)param_1 + 0x38) = *(uint *)((int)param_1 + 0x38) & 0xf000000c;
    puVar6 = (undefined4 *)((int)param_1 + 0x50);
    for (iVar5 = 0x1e; iVar5 != 0; iVar5 = iVar5 + -1) {
      *puVar6 = 0;
      puVar6 = puVar6 + 1;
    }
    local_10 = param_1;
    piVar2 = FUN_004f36a0(*(int *)((int)param_1 + 0x20),(uint *)((int)param_1 + 0x18));
    if ((piVar2 != (int *)0x0) && ((*(byte *)(piVar2 + 0x14) & 8) == 0)) {
      if (*(int *)((int)param_1 + 0x20) == 1) {
        iVar5 = 1;
      }
      else if (*(int *)((int)param_1 + 0x20) == 2) {
        iVar5 = 2;
      }
      else {
        iVar5 = 0;
      }
      if (*(int *)((int)param_1 + 0x24) == iVar5) {
        *(uint *)((int)param_1 + 0x38) = *(uint *)((int)param_1 + 0x38) | 1;
      }
      if ((*(byte *)(piVar2 + 0x14) & 0x10) != 0) {
        *(uint *)((int)param_1 + 0x38) = *(uint *)((int)param_1 + 0x38) | 2;
      }
      puVar3 = FUN_004025b0((void *)piVar2[7],(uint *)&local_10);
      local_4 = 0;
      FUN_004f26d0((void *)((int)param_1 + 0x28),puVar3);
      local_4 = 0xffffffff;
      FUN_00619730();
      iVar5 = thunk_FUN_005f5060((int)param_1 + 0x48);
      *(int *)((int)param_1 + 0x44) = iVar5;
      while (iVar5 != 0) {
        puVar3 = FUN_00403040(*(void **)((int)param_1 + 0x44),&local_10);
        local_4 = 1;
        pvVar4 = FUN_004f5940(*(void **)((int)param_1 + 0x40),puVar3);
        local_4 = 0xffffffff;
        FUN_00619730();
        if (pvVar4 != (void *)0x0) {
          FUN_00484630((int)pvVar4);
          FUN_00433780(param_1,(int *)((int)pvVar4 + 0x44),(uint *)((int)pvVar4 + 0x24));
        }
        iVar5 = *(int *)(*(int *)((int)param_1 + 0x44) + 0x10);
        *(int *)((int)param_1 + 0x44) = iVar5;
      }
      if (0 < *(int *)((int)param_1 + 0xa8)) {
        *(uint *)((int)param_1 + 0x38) = *(uint *)((int)param_1 + 0x38) | 0x80000;
      }
      if (0 < *(int *)((int)param_1 + 0xac)) {
        *(uint *)((int)param_1 + 0x38) = *(uint *)((int)param_1 + 0x38) | 0x100000;
      }
    }
    uVar1 = 1;
  }
  ExceptionList = local_c;
  return uVar1;
}

