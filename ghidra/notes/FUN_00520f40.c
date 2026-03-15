
bool __thiscall FUN_00520f40(void *this,int *param_1)

{
  int *piVar1;
  uint uVar2;
  bool bVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643750;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  piVar1 = (int *)FUN_00504dc0((uint *)((int)this + 0x6c));
  bVar3 = piVar1 == (int *)0x0;
  if (bVar3) goto LAB_00520ffc;
  local_4 = 0;
  uVar2 = (**(code **)(*piVar1 + 4))();
  if ((uVar2 < 0x90) || (0x97 < uVar2)) {
    local_4._0_1_ = 1;
    uVar2 = (**(code **)(*piVar1 + 4))();
    if ((uVar2 < 0x98) || (0x9f < uVar2)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
    if (bVar3) goto LAB_00520fe4;
  }
  else {
LAB_00520fe4:
    bVar3 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  bVar3 = !bVar3;
LAB_00520ffc:
  if (!bVar3) {
    *param_1 = (int)piVar1;
  }
  ExceptionList = pvStack_c;
  return *param_1 != 0;
}

