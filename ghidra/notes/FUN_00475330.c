
int * __fastcall FUN_00475330(int param_1)

{
  int *piVar1;
  int iVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  int *piVar6;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632c43;
  pvStack_c = ExceptionList;
  bVar3 = false;
  piVar6 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_1c);
  piVar1 = (int *)(param_1 + 0x30);
  uVar4 = *(uint *)(param_1 + 0x30) >> 0x18;
  local_4 = 0;
  local_14 = 0x90;
  local_10 = 0x98;
  if ((0x8f < uVar4) && (uVar4 < 0x98)) {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    iVar2 = *(int *)(param_1 + 0x4c);
    FUN_004f26d0(&stack0xffffffc8,piVar1);
    pvVar5 = FUN_00403d30((void *)(iVar2 + 0x2c));
    if ((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x28) & 2) == 0)) {
      iVar2 = *(int *)(param_1 + 0x4c);
      FUN_004f26d0(&stack0xffffffc8,(undefined4 *)(param_1 + 0x2c));
      pvVar5 = FUN_00419e40((void *)(iVar2 + 0x78));
      if ((pvVar5 != (void *)0x0) &&
         ((*(int *)((int)pvVar5 + 0x28) != *piVar1 && ((*(byte *)((int)pvVar5 + 0x38) & 2) == 0))))
      {
        pvVar5 = (void *)FUN_00618b70(0x20);
        local_4._0_1_ = 1;
        if (pvVar5 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          pvVar5 = FUN_004f5b10(pvVar5,(undefined4 *)(param_1 + 0x2c),0);
        }
        local_4 = (uint)local_4._1_3_ << 8;
        if (pvVar5 != (void *)0x0) {
          FUN_004f57b0(local_1c,pvVar5);
          piVar6 = (int *)FUN_004f5cd0(0x201);
          if (piVar6 != (int *)0x0) {
            piVar6[8] = *(int *)(param_1 + 0x20);
            (**(code **)(*piVar6 + 0x24))();
            (**(code **)(*piVar6 + 0x2c))(piVar1);
          }
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_1c);
  ExceptionList = pvStack_c;
  return piVar6;
}

