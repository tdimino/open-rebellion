
int * __thiscall FUN_004e4300(void *this,uint *param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 local_24 [2];
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063e14b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_24);
  bVar1 = false;
  piVar4 = (int *)0x0;
  local_14 = 0x90;
  local_10 = 0x98;
  local_4 = 1;
  if ((0x8f < *param_1 >> 0x18) && (*param_1 >> 0x18 < 0x98)) {
    local_1c = 0xa0;
    uVar2 = *(uint *)((int)this + 0x4c) >> 0x18;
    local_18 = 0xa2;
    if ((0x9f < uVar2) && (uVar2 < 0xa2)) {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) {
      bVar1 = true;
      goto LAB_004e4398;
    }
  }
  bVar1 = false;
LAB_004e4398:
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  if (bVar1) {
    piVar4 = (int *)FUN_004f5cd0(0x214);
    if (piVar4 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004f5b10(pvVar3,(undefined4 *)((int)this + 0x4c),0);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_24,pvVar3);
        piVar4[8] = *(int *)((int)this + 0x30);
        (**(code **)(*piVar4 + 0x24))(local_24);
        (**(code **)(*piVar4 + 0x2c))(param_1);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_24);
  ExceptionList = pvStack_c;
  return piVar4;
}

