
int * __thiscall FUN_004bc040(void *this,uint *param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int *piVar4;
  undefined4 unaff_retaddr;
  undefined4 local_24 [2];
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639b9b;
  pvStack_c = ExceptionList;
  piVar4 = (int *)0x0;
  ExceptionList = &pvStack_c;
  FUN_004f4340(local_24);
  local_14 = 0x30;
  local_10 = 0x3c;
  uVar2 = *(uint *)((int)this + 0x2c) >> 0x18;
  local_4 = 1;
  if ((0x2f < uVar2) && (uVar2 < 0x3c)) {
    bVar1 = true;
    local_1c = 1;
    local_18 = 0xff;
    if ((*param_1 >> 0x18 == 0) || (0xfe < *param_1 >> 0x18)) {
      bVar1 = false;
    }
    FUN_00619730();
    if (bVar1) {
      bVar1 = true;
      goto LAB_004bc0cf;
    }
  }
  bVar1 = false;
LAB_004bc0cf:
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  if (bVar1) {
    piVar4 = (int *)FUN_004f5cd0(0x201);
    if (piVar4 != (int *)0x0) {
      pvVar3 = (void *)FUN_00618b70(0x20);
      local_4._0_1_ = 2;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        pvVar3 = FUN_004f5b10(pvVar3,(undefined4 *)((int)this + 0x2c),0);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (pvVar3 != (void *)0x0) {
        FUN_004f57b0(local_24,pvVar3);
        piVar4[8] = *(int *)((int)this + 0x24);
        (**(code **)(*piVar4 + 0x24))(local_24);
        (**(code **)(*piVar4 + 0x2c))(unaff_retaddr);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_004f4380(local_24);
  ExceptionList = pvStack_c;
  return piVar4;
}

