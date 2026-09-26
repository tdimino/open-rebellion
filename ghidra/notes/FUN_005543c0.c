
uint FUN_005543c0(int *param_1,uint *param_2,int *param_3)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  undefined4 local_38;
  undefined4 local_34;
  undefined4 local_30 [9];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_3;
  puStack_8 = &LAB_00649260;
  local_c = ExceptionList;
  local_38 = 1;
  local_34 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_38);
  local_4 = 0xffffffff;
  FUN_00619730();
  FUN_00551060(local_30);
  local_4 = 1;
  param_3 = (int *)0x3;
  iVar2 = FUN_005511d0(local_30,param_1,(uint)param_1[9] >> 6 & 3,&param_3);
  uVar3 = 0;
  if (iVar2 != 0) {
    uVar3 = FUN_00554490(param_1,param_2,local_30,piVar1);
  }
  local_4 = 0xffffffff;
  FUN_005510b0(local_30);
  ExceptionList = local_c;
  return uVar3;
}

