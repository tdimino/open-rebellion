
int * __thiscall FUN_0049ea40(void *this,int param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 local_20;
  undefined4 local_1c [2];
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006372fb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_0042dbe0(&local_20);
  bVar1 = false;
  local_4 = 0;
  FUN_004f4340(local_1c);
  uVar2 = *(uint *)((int)this + 0x40) >> 0x18;
  piVar4 = (int *)0x0;
  local_4._0_1_ = 1;
  local_14 = 0xa0;
  local_10 = 0xb0;
  if ((0x9f < uVar2) && (uVar2 < 0xb0)) {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    piVar4 = (int *)FUN_004f5cd0(param_1);
    if (piVar4 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004f5b10(pvVar3,(undefined4 *)((int)this + 0x40),0);
      }
      local_4._0_1_ = 1;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_1c,pvVar3);
        piVar4[8] = *(int *)((int)this + 0x2c);
        (**(code **)(*piVar4 + 0x24))(local_1c);
      }
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f4380(local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return piVar4;
}

