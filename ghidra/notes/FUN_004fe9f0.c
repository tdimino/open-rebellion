
bool __thiscall FUN_004fe9f0(void *this,int *param_1,undefined4 param_2,undefined4 param_3)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  bool bVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640568;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = FUN_00524b10();
  bVar4 = iVar2 != 0;
  local_4 = 0;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x14) || (0x1b < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = (**(code **)(*(int *)this + 0x1cc))(param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  ExceptionList = pvStack_c;
  return bVar4;
}

