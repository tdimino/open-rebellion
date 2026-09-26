
bool __thiscall FUN_00588700(void *this,undefined4 *param_1,undefined4 param_2)

{
  int iVar1;
  bool bVar2;
  int local_20 [3];
  int local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f9a8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  bVar2 = true;
  if (*(int *)((int)this + 0x30) != 0) {
    FUN_00587360(local_20,this);
    local_4 = 0;
    local_14 = FUN_0053e290(*(int *)((int)this + 0x30) + -1);
    iVar1 = FUN_00587b30(this,local_20,param_2);
    bVar2 = iVar1 != 0;
    *param_1 = local_10;
    local_4 = 0xffffffff;
    FUN_005873b0(local_20);
  }
  ExceptionList = local_c;
  return bVar2;
}

