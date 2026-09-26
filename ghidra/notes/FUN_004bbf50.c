
int * __thiscall FUN_004bbf50(void *this,int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639b73;
  pvStack_c = ExceptionList;
  bVar1 = false;
  piVar4 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_1c);
  uVar2 = *(uint *)((int)this + 0x2c) >> 0x18;
  local_4 = 0;
  local_14 = 0x30;
  local_10 = 0x3c;
  if ((0x2f < uVar2) && (uVar2 < 0x3c)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar4 = (int *)FUN_004f5cd0(param_1);
    if (piVar4 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 1;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004f5b10(pvVar3,(undefined4 *)((int)this + 0x2c),0);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_1c,pvVar3);
        piVar4[8] = *(int *)((int)this + 0x24);
        (**(code **)(*piVar4 + 0x24))(local_1c);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_1c);
  ExceptionList = pvStack_c;
  return piVar4;
}

