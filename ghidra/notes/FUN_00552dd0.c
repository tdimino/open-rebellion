
uint __thiscall FUN_00552dd0(void *this,int param_1,int param_2,uint *param_3)

{
  bool bVar1;
  undefined3 extraout_var;
  uint uVar2;
  uint uVar3;
  void *local_1c;
  void *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00648ee8;
  local_c = ExceptionList;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  local_1c = (void *)0x0;
  local_18 = (void *)0x0;
  ExceptionList = &local_c;
  bVar1 = FUN_005530d0(this,param_3,(int *)&local_1c,&local_18);
  uVar2 = 0;
  if (CONCAT31(extraout_var,bVar1) != 0) {
    uVar2 = FUN_00550eb0(local_1c,param_1,param_2);
  }
  uVar3 = 0;
  if (uVar2 != 0) {
    uVar3 = FUN_00550fb0(local_18,param_1,param_2);
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return uVar3;
}

