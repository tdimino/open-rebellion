
bool __thiscall FUN_005887a0(void *this,int *param_1,undefined4 param_2)

{
  int iVar1;
  bool bVar2;
  int local_18 [2];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064f9c8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = 0;
  bVar2 = *(int *)((int)this + 0x2c) != 0;
  if (*(int *)((int)this + 0x2c) != 0) {
    FUN_005874c0(local_18,this);
    local_4 = 0;
    iVar1 = FUN_00587b50(this,local_18,param_2);
    if ((iVar1 == 0) || (!bVar2)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    local_4 = 0xffffffff;
    *param_1 = local_10 / *(int *)((int)this + 0x2c);
    FUN_00587500(local_18);
  }
  ExceptionList = local_c;
  return bVar2;
}

