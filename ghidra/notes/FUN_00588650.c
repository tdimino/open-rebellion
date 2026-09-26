
bool __thiscall FUN_00588650(void *this,undefined4 *param_1,undefined4 *param_2,undefined4 param_3)

{
  int iVar1;
  bool bVar2;
  uint local_24 [3];
  int local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f988;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  *param_2 = 0;
  bVar2 = true;
  if (*(int *)((int)this + 0x34) != 0) {
    FUN_00587410(local_24,this);
    local_4 = 0;
    local_18 = FUN_0053e290(*(int *)((int)this + 0x34) + -1);
    iVar1 = FUN_005875e0(this,local_24,param_3);
    local_4 = 0xffffffff;
    bVar2 = iVar1 != 0;
    *param_1 = local_14;
    *param_2 = local_10;
    FUN_00587460(local_24);
  }
  ExceptionList = local_c;
  return bVar2;
}

