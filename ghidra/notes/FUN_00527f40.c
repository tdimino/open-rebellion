
int __thiscall FUN_00527f40(void *this,undefined4 *param_1,void *param_2)

{
  int iVar1;
  int *piVar2;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006444e8;
  local_c = ExceptionList;
  local_1c = 1;
  local_18 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_1c);
  local_4 = 0xffffffff;
  FUN_00619730();
  iVar1 = FUN_004f9860(this,(int)param_1,param_2);
  if ((iVar1 != 0) && (*(int *)((int)param_2 + 4) == -1)) {
    piVar2 = (int *)FUN_0052be30(this);
    if (piVar2 == (int *)0x0) {
      local_1c = 0xa0;
      local_18 = 1;
      local_4 = 1;
      FUN_00520580(param_2,&local_1c);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    else {
      FUN_00541eb0(&local_1c,param_1);
      local_4 = 2;
      local_10 = 0x203;
      iVar1 = (**(code **)(*piVar2 + 100))(&local_1c,param_2);
      local_4 = 0xffffffff;
      FUN_00541ea0();
    }
  }
  ExceptionList = local_c;
  return iVar1;
}

