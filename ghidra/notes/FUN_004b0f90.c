
int * __fastcall FUN_004b0f90(int param_1)

{
  int iVar1;
  void *pvVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 local_14 [2];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638ed3;
  pvStack_c = ExceptionList;
  piVar4 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_14);
  iVar1 = *(int *)(param_1 + 0xc);
  local_4 = 0;
  FUN_004f26d0(&stack0xffffffd4,(undefined4 *)(param_1 + 0x28));
  pvVar2 = FUN_00419e40((void *)(iVar1 + 0x78));
  if (pvVar2 != (void *)0x0) {
    pvVar3 = (void *)FUN_00618b70(0x20);
    local_4._0_1_ = 1;
    if (pvVar3 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_004f5b10(pvVar3,(undefined4 *)(param_1 + 0x28),0);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    if (pvVar3 != (void *)0x0) {
      FUN_004f57b0(local_14,pvVar3);
      piVar4 = (int *)FUN_004f5cd0(0x201);
      if (piVar4 != (int *)0x0) {
        piVar4[8] = *(int *)(param_1 + 4);
        (**(code **)(*piVar4 + 0x24))();
        (**(code **)(*piVar4 + 0x2c))((int)pvVar2 + 0x2c);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_14);
  ExceptionList = pvStack_c;
  return piVar4;
}

