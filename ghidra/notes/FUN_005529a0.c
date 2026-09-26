
int __thiscall FUN_005529a0(void *this,uint *param_1,int *param_2,void *param_3)

{
  int *piVar1;
  int iVar2;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_2;
  puStack_8 = &LAB_00648e48;
  local_c = ExceptionList;
  iVar2 = 1;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  param_2 = (int *)0x0;
  if (piVar1[1] == -1) {
    iVar2 = FUN_00552f40(this,param_1,piVar1,(int *)&param_2);
  }
  if ((iVar2 != 0) && (piVar1[1] == -1)) {
    iVar2 = FUN_00550200(param_3,param_2,(uint *)((int)this + 0x20),0,piVar1);
  }
  ExceptionList = local_c;
  return iVar2;
}

