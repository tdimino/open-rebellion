
bool __thiscall FUN_004f87b0(void *this,int param_1,undefined4 param_2,int param_3)

{
  bool bVar1;
  int iVar2;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063fe38;
  local_c = ExceptionList;
  bVar1 = true;
  ExceptionList = &local_c;
  FUN_004fd450(&local_28,param_3);
  local_4 = 0;
  local_20 = 3;
  local_24 = 1;
  if (param_1 == 1) {
    iVar2 = 1;
  }
  else {
    if (param_1 != 2) goto LAB_004f881d;
    iVar2 = 2;
  }
  iVar2 = FUN_004f9510(this,iVar2,&local_28);
  bVar1 = iVar2 != 0;
LAB_004f881d:
  local_4 = 0xffffffff;
  FUN_004fd4d0(&local_28);
  ExceptionList = local_c;
  return bVar1;
}

