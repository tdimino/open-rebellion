
bool __thiscall FUN_0058a020(void *this,undefined4 param_1)

{
  void *this_00;
  int iVar1;
  bool bVar2;
  uint local_1c [2];
  int local_14 [2];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fbf0;
  local_c = ExceptionList;
  this_00 = (void *)((int)this + 8);
  ExceptionList = &local_c;
  FUN_00589250(local_14,this_00);
  local_4 = 0;
  iVar1 = FUN_00587fa0(this_00,local_14,param_1);
  bVar2 = iVar1 != 0;
  if (*(int *)((int)this + 0x4c) == 0) {
    FUN_005895d0(local_1c,this_00);
    local_4 = CONCAT31(local_4._1_3_,1);
    if (*(int *)((int)this + 0x20) != 0) {
      iVar1 = FUN_005875e0(this_00,local_1c,param_1);
      if ((iVar1 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
    if (*(int *)((int)this + 0x24) != 0) {
      iVar1 = FUN_00587600(this_00,local_1c,param_1);
      if ((iVar1 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      iVar1 = FUN_00587620(this_00,local_1c,param_1);
      if ((iVar1 == 0) || (!bVar2)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
    }
    local_4 = local_4 & 0xffffff00;
    FUN_00589610(local_1c);
  }
  local_4 = 0xffffffff;
  FUN_00589290(local_14);
  ExceptionList = local_c;
  return bVar2;
}

