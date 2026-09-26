
int __thiscall FUN_00550c90(void *this,int param_1,int param_2,void *param_3,void *param_4)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined4 local_44 [7];
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648af8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  iVar2 = FUN_0054fa40(local_28);
  local_4 = 0;
  FUN_0054fae0(param_3,iVar2);
  local_4 = 0xffffffff;
  FUN_0054fa90(local_28);
  iVar2 = FUN_0054fa40(local_28);
  local_4 = 1;
  FUN_0054fae0(param_4,iVar2);
  local_4 = 0xffffffff;
  FUN_0054fa90(local_28);
  FUN_0054fa40(local_44);
  local_4 = 2;
  iVar2 = FUN_00550af0((int)this + 0x28,param_1 + 4,param_2,local_44);
  if (iVar2 != 0) {
    bVar1 = FUN_0054fbb0(param_3,(int)local_44);
    if (CONCAT31(extraout_var,bVar1) != 0) {
      FUN_0054fae0(param_3,(int)local_44);
    }
    if (iVar2 != 0) {
      FUN_0054fae0(param_4,(int)local_44);
      if (iVar2 != 0) {
        iVar2 = FUN_00550af0((int)this + 0x128,param_1 + 0x74,param_2,local_44);
        if (iVar2 != 0) {
          bVar1 = FUN_0054fbb0(param_3,(int)local_44);
          if (CONCAT31(extraout_var_00,bVar1) != 0) {
            FUN_0054fae0(param_3,(int)local_44);
          }
        }
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_0054fa90(local_44);
  ExceptionList = local_c;
  return iVar2;
}

