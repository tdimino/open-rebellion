
bool __thiscall FUN_0050c6c0(void *this,int *param_1,int param_2,void *param_3)

{
  bool bVar1;
  uint uVar2;
  int iVar3;
  bool bVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_00641d80;
  pvStack_c = ExceptionList;
  bVar4 = true;
  bVar1 = false;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  uVar2 = (**(code **)(*param_1 + 4))();
  if ((uVar2 < 8) || (0xf < uVar2)) {
    local_4._0_1_ = 1;
    uVar2 = (**(code **)(*param_1 + 4))();
    if ((0x1b < uVar2) && (uVar2 < 0x20)) {
      bVar1 = true;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if (bVar1) goto LAB_0050c74f;
  }
  else {
LAB_0050c74f:
    if ((((param_2 == 0) && ((*(uint *)((int)this + 0x88) & 0x1000) != 0)) &&
        ((param_1[0x14] & 0x10U) != 0)) && ((param_1[0x14] & 0x40U) != 0)) {
      uVar2 = FUN_00509ad0((int)this);
      if (((uint)param_1[9] >> 6 & 3) != uVar2) {
        bVar1 = true;
        goto LAB_0050c795;
      }
    }
  }
  bVar1 = false;
LAB_0050c795:
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    uVar2 = (uint)param_1[9] >> 6 & 3;
    if (((uVar2 == 0) || (3 < uVar2)) || (uVar2 == 3)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    if ((*(uint *)((int)this + 0x78) & 0xc) == 0) {
      iVar3 = FUN_0050a0b0(this,uVar2,param_3);
      if ((iVar3 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
    if (bVar4) {
      bVar4 = uVar2 == (*(uint *)((int)this + 0x78) >> 2 & 3);
    }
  }
  ExceptionList = pvStack_c;
  return bVar4;
}

