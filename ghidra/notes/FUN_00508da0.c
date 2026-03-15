
bool __thiscall FUN_00508da0(void *this,int *param_1,int param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  uint uVar3;
  bool bVar4;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641878;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar2 = FUN_00524b10();
  bVar4 = iVar2 != 0;
  bVar1 = false;
  local_4 = 0;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((0x13 < uVar3) && (uVar3 < 0x20)) {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0050b230(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0050b310(this,param_3);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    if (param_2 != 0) {
      iVar2 = FUN_0050c580(this,param_1,param_3);
      if ((iVar2 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
  }
  local_4 = 1;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 0x10) || (0x13 < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0050b230(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
  }
  local_4 = 2;
  uVar3 = (**(code **)(*param_1 + 4))();
  if ((uVar3 < 8) || (0xf < uVar3)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    iVar2 = FUN_0050b230(this,param_3);
    if ((iVar2 == 0) || (!bVar4)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    iVar2 = FUN_0050b310(this,param_3);
    if ((iVar2 == 0) || (!bVar1)) {
      bVar4 = false;
    }
    else {
      bVar4 = true;
    }
    if (param_2 != 0) {
      iVar2 = FUN_0050c350(this,param_3);
      if ((iVar2 == 0) || (!bVar4)) {
        bVar4 = false;
      }
      else {
        bVar4 = true;
      }
    }
  }
  ExceptionList = pvStack_c;
  return bVar4;
}

